
RUSTFLAGS="-C opt-level=z -C relocation-model=static" cargo build -Z build-std --release --target dos.json
