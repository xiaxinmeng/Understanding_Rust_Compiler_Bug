
$ rustc +nightly main.rs --target wasm32-wasi
$ wasmtime main.wasm
thread 'main' panicked at 'attempt to add with overflow', main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
...
