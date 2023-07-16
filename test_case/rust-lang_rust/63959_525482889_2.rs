ps
$env:RUSTFLAGS = "-C target-cpu=znver1"
cargo run --release
