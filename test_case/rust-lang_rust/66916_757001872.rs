
$ rustup target add wasm32-unknown-emscripten
    ...

$ cargo install wasm-bindgen-cli
    ...

$ cargo build --target wasm32-unknown-emscripten
    Compiling ...

$ wasm-bindgen target/wasm32-unknown-emscripten/debug/bindgenhello.wasm --out-dir target/wasm32-unknown-emscripten/debug

$ webpack bundle

(node:160144) [DEP_WEBPACK_COMPILATION_ASSETS] DeprecationWarning: Compilation.assets will be frozen in future, all modifications are deprecated.
BREAKING CHANGE: No more changes should happen to Compilation.assets after sealing the Compilation.
        Do changes to assets earlier, e. g. in Compilation.hooks.processAssets.
        Make sure to select an appropriate stage from Compilation.PROCESS_ASSETS_STAGE_*.
assets by status 132 KiB [cached] 1 asset
assets by status 20.1 KiB [emitted]
  asset index.js 12.2 KiB [emitted] (name: main)
  asset target_wasm32-unknown-emscripten_debug_bindgenhello_js.index.js 7.89 KiB [emitted]
asset index.html 226 bytes [compared for emit]
runtime modules 6.95 KiB 9 modules
cacheable modules 703 bytes (javascript) 132 KiB (webassembly)
  modules by path ./target/wasm32-unknown-emscripten/debug/*.js 134 bytes
    ./target/wasm32-unknown-emscripten/debug/bindgenhello.js 85 bytes [built] [code generated]
    ./target/wasm32-unknown-emscripten/debug/bindgenhello_bg.js 49 bytes [built] [code generated]
  ./index.js 249 bytes [built] [code generated]
  ./target/wasm32-unknown-emscripten/debug/bindgenhello_bg.wasm 320 bytes (javascript) 132 KiB (webassembly) [built] [code generated]

ERROR in ./target/wasm32-unknown-emscripten/debug/bindgenhello_bg.wasm
Module not found: Error: Can't resolve 'env' in '/home/revlin/Public/edu/rust/target/wasm32-unknown-emscripten/debug'
 @ ./target/wasm32-unknown-emscripten/debug/bindgenhello.js 1:0-47
 @ ./index.js 2:13-76

ERROR in ./target/wasm32-unknown-emscripten/debug/bindgenhello_bg.wasm
Module not found: Error: Can't resolve 'wasi_snapshot_preview1' in '/home/revlin/Public/edu/rust/target/wasm32-unknown-emscripten/debug'
 @ ./target/wasm32-unknown-emscripten/debug/bindgenhello.js 1:0-47
 @ ./index.js 2:13-76

webpack 5.12.1 compiled with 2 errors in 349 ms
