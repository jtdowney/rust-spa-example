use std::{fs::File, path::Path, time::Duration};

use anyhow::format_err;
use argh::FromArgs;
use askama::Template;
use axum::{
    extract::{Path as ExtractPath, State},
    http::StatusCode,
    routing::{get, get_service},
    Json, Router,
};
use config::IS_DEVELOPMENT;
use manifest::Manifest;
use once_cell::sync::OnceCell;
use serde::Serialize;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool},
};
use tower_http::services::ServeDir;

mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

mod manifest;

const ASSET_ENTRIES: [&str; 1] = ["app/main.tsx"];

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");
static SCRIPTS: OnceCell<Vec<String>> = OnceCell::new();
static STYLES: OnceCell<Vec<String>> = OnceCell::new();

#[derive(FromArgs)]
/// Run packlist server
struct Args {
    /// path to static assets
    #[argh(option, default = "String::from(\"dist\")")]
    assets: String,
    /// address to bind to
    #[argh(option, default = "String::from(\"127.0.0.1\")")]
    bind: String,
    /// port to bind to
    #[argh(option, default = "3000")]
    port: u16,
    /// path to database
    #[argh(option, default = "String::from(\"db/sqlite3.db\")")]
    db: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    development: bool,
    scripts: &'a [String],
    styles: &'a [String],
}

#[derive(Clone, Serialize)]
struct Post {
    id: i64,
    slug: String,
    title: String,
    author: String,
    body: String,
}

async fn spa_index<'a>() -> IndexTemplate<'a> {
    if IS_DEVELOPMENT {
        IndexTemplate {
            development: IS_DEVELOPMENT,
            scripts: &[],
            styles: &[],
        }
    } else {
        IndexTemplate {
            development: IS_DEVELOPMENT,
            scripts: SCRIPTS.get().unwrap().as_slice(),
            styles: STYLES.get().unwrap().as_slice(),
        }
    }
}

async fn list_posts(State(pool): State<SqlitePool>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "select * from posts;")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(posts))
}

async fn get_post(
    State(pool): State<SqlitePool>,
    ExtractPath(slug): ExtractPath<String>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(Post, "select * from posts where slug=?;", slug)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(post))
}

fn load_asset_manifest<P: AsRef<Path>>(assets_path: P) -> anyhow::Result<()> {
    let manifest_path = assets_path.as_ref().join("manifest.json");
    let manifest_file = File::open(manifest_path)?;
    let manifest: Manifest = serde_json::from_reader(manifest_file)?;

    let mut scripts = vec![];
    let mut styles = vec![];
    for (file, entry) in manifest {
        if ASSET_ENTRIES.contains(&file.as_str()) {
            scripts.push(entry.file);
            styles.extend_from_slice(&entry.css);
        }
    }

    SCRIPTS
        .set(scripts)
        .map_err(|_| format_err!("unable to set scripts"))?;
    STYLES
        .set(styles)
        .map_err(|_| format_err!("unable to set styles"))?;

    Ok(())
}

fn create_router<P: AsRef<Path>>(assets_path: P) -> Router<SqlitePool> {
    let assets_path = assets_path.as_ref().join("assets");
    let assets_server = ServeDir::new(assets_path);

    let api_routes = Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/:slug", get(get_post));

    Router::new()
        .nest("/api", api_routes)
        .nest_service("/assets", get_service(assets_server))
        .fallback(spa_index)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let options = SqliteConnectOptions::new()
        .filename(&args.db)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));
    let pool = SqlitePool::connect_with(options).await?;

    println!("Running migrations...");
    MIGRATOR.run(&pool).await?;

    if !IS_DEVELOPMENT {
        load_asset_manifest(&args.assets)?;
    }

    let app = create_router(&args.assets).with_state(pool);
    let addr = format!("{}:{}", args.bind, args.port);

    println!("Listening on http://{addr}");
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
