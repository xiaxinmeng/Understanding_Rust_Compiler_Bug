sh
$ rustup toolchain install nightly-2021-03-10
$ rustup default nightly-2021-03-10
$ rustup target add wasm32-wasi
$ export CARGO_TARGET_WASM32_WASI_LINKER=/path/to/llvm-11/bin/lld
$ cargo build --target wasm32-wasi
$ echo "YES" > path.txt
$ wasmtime run --mapdir=absolute::. target/wasm32-wasi/debug/rust-repro.wasm
[
    89,
    69,
    83,
    10,
]
