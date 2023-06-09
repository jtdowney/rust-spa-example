dev:
    parallel --lb ::: 'cargo run' 'pnpm run dev'

lint:
    cargo clippy
    pnpm run lint

check:
    cargo check
    pnpm run check

build:
    cargo build --release
    pnpm run build

dockerize:
    docker buildx build -t rust-spa-example .

clean:
    cargo clean
    rm -rf dist