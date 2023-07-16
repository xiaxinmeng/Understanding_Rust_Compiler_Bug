
$ RUSTFLAGS="-C link-arg=--export-table" cargo build --target wasm32-unknown-unknown --release
   Compiling test_lld v0.1.0 (file:///private/tmp/test_lld)
    Finished release [optimized] target(s) in 0.18s
$ ls -alh target/wasm32-unknown-unknown/release/test_lld.wasm
-rwxr-xr-x  2 marko  wheel   270B 13 Aug 21:30 target/wasm32-unknown-unknown/release/test_lld.wasm
