# rust-spa-example

An example of a Rust + Axum backend with a TypeScript + SolidJS frontend using Vite and Tailwind. Vite allows for hot module reloading during development for a fast dev cycle.

## Running

```bash
just dev
```

## Deployment

Build a ~80MB docker image that contains a production build of the backend and frontend.

```bash
just dockerize
```
