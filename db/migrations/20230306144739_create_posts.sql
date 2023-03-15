create table posts(
    id integer primary key autoincrement,
    slug text not null,
    title text not null,
    author text not null,
    body text not null
);