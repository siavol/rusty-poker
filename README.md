# Rusty Poker

## Pre-requisites

1. [Rust](https://www.rust-lang.org/)
2. [trunk](https://trunkrs.dev/)
    ```bash
    cargo install trunk
    ```
3. `wasm32-unknown-unknown` target
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

## Run

Build the client:
```bash
cd client
trunk build -d ../server/static
```

Run server:
```bash
cd server
RUST_LOG=debug PORT=8080 cargo run
```