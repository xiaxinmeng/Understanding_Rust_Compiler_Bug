plain
test [ui] src/test/ui/transmutability/primitives/numbers.rs ... ok

failures:

---- [ui] src/test/ui/thread-available_parallelism.rs stdout ----
error: test run failed!
status: exit status: 101
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-available_parallelism/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[74]:0x4cc8)
    at rust_panic (<anonymous>:wasm-function[71]:0x4c8e)
    at _ZN3std9panicking20rust_panic_with_hook17hc73a70925786882dE (<anonymous>:wasm-function[70]:0x4c5e)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h7f5f3ae987f1af25E (<anonymous>:wasm-function[60]:0x435b)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h185a02d230e881b2E (<anonymous>:wasm-function[59]:0x4288)
    at rust_begin_unwind (<anonymous>:wasm-function[65]:0x48b2)
    at _ZN4core9panicking9panic_fmt17h1e2bb86f52aeefd3E (<anonymous>:wasm-function[120]:0x55f0)
    at _ZN4core6result13unwrap_failed17hc4e20a91daf996c2E (<anonymous>:wasm-function[145]:0x66db)
    at _ZN28thread_available_parallelism4main17h03326d289402e501E (<anonymous>:wasm-function[5]:0x335)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hf58fe416dcfc238fE (<anonymous>:wasm-function[0]:0x1ef)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h908b31335600f762E (<anonymous>:wasm-function[1]:0x211)
    at _ZN3std2rt19lang_start_internal17h8a5b5e510fe566d4E (<anonymous>:wasm-function[48]:0x38e8)
    at main (<anonymous>:wasm-function[6]:0x36e)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47



failures:
