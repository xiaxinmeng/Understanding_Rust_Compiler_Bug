shell
RUSTFLAGS="-C llvm-args=--pass-remarks-output=filename -Cdebuginfo=2" cargo build --release
