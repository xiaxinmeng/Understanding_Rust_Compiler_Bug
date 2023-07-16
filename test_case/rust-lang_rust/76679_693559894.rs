bash
cargo bisect-rustc --start=2020-06-30 --end=2020-07-30 --target wasm32-wasi -- build --target wasm32-wasi
