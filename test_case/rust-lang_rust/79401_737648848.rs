shell
RUSTFLAGS="-Zinstrument-coverage" cargo build --release -Z build-std --target x86_64-unknown-linux-gnu
