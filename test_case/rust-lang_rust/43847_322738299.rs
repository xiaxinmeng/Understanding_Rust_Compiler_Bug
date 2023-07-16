sh
RUST_LOG=debug ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc -g --crate-type=lib src/test/run-pass/backtrace.rs
