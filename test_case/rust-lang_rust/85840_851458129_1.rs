sh
$ rustup toolchain install nightly-2021-03-11
$ rustup default nightly-2021-03-11
$ rustup target add wasm32-wasi
$ export CARGO_TARGET_WASM32_WASI_LINKER=/path/to/llvm-11/bin/lld
$ cargo build --target wasm32-wasi
$ echo "YES" > path.txt
$ wasmtime run --mapdir=absolute::. target/wasm32-wasi/debug/rust-repro.wasm
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: Other, error: "failed to find a pre-opened file descriptor through which \"/absolute/path.txt\" could be opened" }', src/main.rs:4:44
