plain
---- [ui] tests/ui/panics/nested_panic_caught.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/nested_panic_caught" && RUST_TEST_THREADS="8" "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/nested_panic_caught/a.wasm"
stdout: none
RuntimeError: unreachable
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[58]:0x3b7c)
    at rust_panic (<anonymous>:wasm-function[55]:0x3b42)
    at _ZN3std9panicking20rust_panic_with_hook17h6fed28fb77f3727bE (<anonymous>:wasm-function[54]:0x3b34)
    at _ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h4f81d0ea7620f924E (<anonymous>:wasm-function[2]:0x1b8)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hdb7aefd96411b68eE (<anonymous>:wasm-function[1]:0x17d)
    at _ZN3std9panicking11begin_panic17hdb3a81ae8305fb6eE (<anonymous>:wasm-function[5]:0x242)
    at _ZN3std9panicking3try17hf397ba370344330eE (<anonymous>:wasm-function[6]:0x24c)
    at _ZN19nested_panic_caught4main17h8d32e432ac72d946E (<anonymous>:wasm-function[11]:0x328)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h87987b987998a558E (<anonymous>:wasm-function[3]:0x1c4)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17ha13efbf6c3ba53c9E (<anonymous>:wasm-function[4]:0x1e6)
    at _ZN3std2rt19lang_start_internal17he8517c50f5abaf06E (<anonymous>:wasm-function[38]:0x2f56)
    at main (<anonymous>:wasm-function[12]:0x361)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47



failures:
