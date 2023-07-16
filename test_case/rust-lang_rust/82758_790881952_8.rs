
$ wasm-dis target/wasm32-wasi/release/regress.wasm | grep -A2 __wasilibc_find_relpath_alloc | head -3
