
git clone https://github.com/PoC-Consortium/scavenger
cd scavenger
cargo check --features=simd           # succeeds
cargo +nightly check --features=simd  # fails