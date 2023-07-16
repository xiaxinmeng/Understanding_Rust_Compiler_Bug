 rust
cargo build --release
RUSTFLAGS="-Z sanitizer=leak" cargo build --target x86_64-unknown-linux-gnu --release
