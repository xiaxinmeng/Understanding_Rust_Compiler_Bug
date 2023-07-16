
RUSTFLAGS="-C target-cpu=pentium" cargo rustc --release --target i686-unknown-linux-gnu -- --emit asm
