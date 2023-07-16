text
rustup override set 1.34.0
cargo run # <- errors
vim Cargo.toml # comment out the edition line
cargo run # <- works
