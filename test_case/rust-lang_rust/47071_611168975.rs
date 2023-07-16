shell
export RUSTFLAGS="-Clink-arg=-fuse-ld=gold -C codegen-units=16 -C passes=sancov -C llvm-args=-sanitizer-coverage-level=3 -Z sanitizer=address -C opt-level=3"
cargo build --target x86_64-unknown-linux-gnu
