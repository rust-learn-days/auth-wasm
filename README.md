## add wasm rust

1. rustup target add wasm32-unknown-unknown
2. cargo install wasm-gc
3. cargo new add --lib
4. cargo build --target wasm32-unknown-unknown --release
