
cargo build -p crate-name <- succeeds
touch src/lib.rs
cargo build -p crate-name <- ICE
cargo clean -p crate-name
cargo build -p crate-name <- succeeds
