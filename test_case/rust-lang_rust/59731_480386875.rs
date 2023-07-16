

LinkError: WebAssembly Instantiation: Import #24 module="./wasm-bindgen-test" function="__wbg_buffer_859770b344faa681" error: function import requires a callable
    at Object.<anonymous> (C:\git\replay-web\libsrc\rust\le-core\target\wasm32-unknown-unknown\wbg-tmp\wasm-bindgen-test_bg.js:8:22)
    at Module._compile (internal/modules/cjs/loader.js:722:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:733:10)
    at Module.load (internal/modules/cjs/loader.js:620:32)
    at tryModuleLoad (internal/modules/cjs/loader.js:560:12)
    at Function.Module._load (internal/modules/cjs/loader.js:552:3)
    at Module.require (internal/modules/cjs/loader.js:658:17)
    at require (internal/modules/cjs/helpers.js:22:18)
    at Object.<anonymous> (C:\git\replay-web\libsrc\rust\le-core\target\wasm32-unknown-unknown\wbg-tmp\wasm-bindgen-test.js:818:8)
    at Module._compile (internal/modules/cjs/loader.js:722:30)
error: test failed, to rerun pass '--lib'
Error: Running Wasm tests with wasm-bindgen-test failed
Caused by: failed to execute `cargo test`: exited with exit code: 1
