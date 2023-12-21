build:
    cargo build --target wasm32-unknown-unknown --release

ci:
  cargo test --all
  cargo clippy --all --all-targets -- --deny warnings
  cargo fmt --all -- --check
  cargo update --locked
