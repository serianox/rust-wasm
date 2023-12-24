build:
    cargo build --release
    wasm-tools print target/wasm32-unknown-unknown/release/hello_world.wasm >target/wasm32-unknown-unknown/release/hello_world.wat

ci:
  #cargo test --all
  cargo clippy --all -- --deny warnings
  cargo fmt --all -- --check
  cargo update --locked
