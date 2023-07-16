sh
env RUSTFLAGS="-L dependency=target/debug/deps" cargo build --verbose --example=example --target=x86_64-unknown-linux-musl
