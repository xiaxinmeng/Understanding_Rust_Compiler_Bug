
cargo install clippy --git https://github.com/rust-lang-nursery/rust-clippy.git --branch master --force && cargo clean && RUST_BACKTRACE=full cargo clippy --verbose
