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

## Dev

Start the server:
```bash
cd server
RUST_LOG=debug cargo run
```

Open one more terminal. Start the client with the proxy to server (and in watch client changes mode):
```bash
cd client
trunk serve --proxy-backend=http://localhost:3000/api/ 
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
RUST_LOG=debug cargo run
```