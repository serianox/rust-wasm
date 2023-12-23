build:
    cargo build --target wasm32-unknown-unknown --release
    wasm-tools print target/wasm32-unknown-unknown/release/hello_world.wasm >target/wasm32-unknown-unknown/release/hello_world.wat

ci:
  cargo test --all
  cargo clippy --all --all-targets -- --deny warnings
  cargo fmt --all -- --check
  cargo update --locked
