FROM rust as rust-chef
RUN cargo install cargo-chef

FROM rust-chef as planner

WORKDIR /usr/src
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust-chef as builder

WORKDIR /usr/src
COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM node:18 AS asset-builder

WORKDIR /usr/src
COPY . .
RUN npm install -g pnpm && \
    pnpm install && \
    pnpm run build

FROM debian:bullseye-slim

COPY --from=builder /usr/src/target/release/rust-spa-example /usr/bin
COPY --from=asset-builder /usr/src/dist/ /srv
USER nobody
CMD ["/usr/bin/rust-spa-example", "--assets", "/srv", "--bind", "0.0.0.0"]