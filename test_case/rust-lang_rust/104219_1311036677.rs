
 failures:

---- [ui] src/test/ui/async-await/track-caller/panic-track-caller.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/track-caller/panic-track-caller/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[94]:0x5a97)
    at rust_panic (<anonymous>:wasm-function[91]:0x5a5d)
    at _ZN3std9panicking20rust_panic_with_hook17h0c233e78e6112adfE (<anonymous>:wasm-function[90]:0x5a2d)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hf994fd019a2fd642E (<anonymous>:wasm-function[77]:0x4a54)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17ha238eeecdfc892b1E (<anonymous>:wasm-function[76]:0x49b9)
    at rust_begin_unwind (<anonymous>:wasm-function[85]:0x5681)
    at _ZN4core9panicking9panic_fmt17ha1487ae10087c69cE (<anonymous>:wasm-function[140]:0x63bf)
    at _ZN4core9panicking5panic17heb919bbe7cf670eaE (<anonymous>:wasm-function[143]:0x680c)
    at _ZN3std5panic12catch_unwind17h79eba0ed8ab51e6aE (<anonymous>:wasm-function[2]:0x230)
    at _ZN18panic_track_caller4main17h0fa14fb0b64e685eE (<anonymous>:wasm-function[14]:0x66a)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h363919328fe77852E (<anonymous>:wasm-function[0]:0x1cf)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h226d7bc4ae73ce06E (<anonymous>:wasm-function[1]:0x1f1)
    at _ZN3std2rt19lang_start_internal17h8bbe173c9dd6e6e6E (<anonymous>:wasm-function[64]:0x3f60)
    at main (<anonymous>:wasm-function[15]:0x6b2)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
------------------------------------------



failures:
    [ui] src/test/ui/async-await/track-caller/panic-track-caller.rs

test result: FAILED. 13105 passed; 1 failed; 704 ignored; 0 measured; 0 filtered out; finished in 92.50s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
