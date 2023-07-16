
$ cargo build --target wasm32-unknown-unknown --release
   Compiling test_lld v0.1.0 (file:///private/tmp/test_lld)
    Finished release [optimized] target(s) in 0.27s
$ ls -alh target/wasm32-unknown-unknown/release/test_lld.wasm
-rwxr-xr-x  2 marko  wheel   242B 13 Aug 21:29 target/wasm32-unknown-unknown/release/test_lld.wasm
