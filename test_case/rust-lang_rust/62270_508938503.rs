
failures:

---- [ui] ui/async-await/async-fn-size-moved-locals.rs stdout ----

error: test run failed!
status: exit code: 101
command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-moved-locals/a.wasm"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
RuntimeError: unreachable
    at __rust_start_panic (wasm-function[68]:1)
    at rust_panic (wasm-function[64]:39)
    at _ZN3std9panicking20rust_panic_with_hook17h2d6e43efc6bf77fdE (wasm-function[59]:346)
    at _ZN3std9panicking18continue_panic_fmt17h9016955e9027da33E (wasm-function[58]:151)
    at _ZN3std9panicking15begin_panic_fmt17h52d189e875ddbafaE (wasm-function[33]:108)
    at _ZN26async_fn_size_moved_locals4main17h2d699e92deec06acE (wasm-function[4]:144)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hdba8efe9f3805544E (wasm-function[1]:25)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4c4b5cb95474f728E (wasm-function[48]:8)
    at _ZN3std9panicking3try7do_call17hd33565ab64c5123fE (wasm-function[56]:20)
    at __rust_maybe_catch_panic (wasm-function[67]:5)
    at _ZN3std2rt19lang_start_internal17hb4e78568bfc14e3aE (wasm-function[65]:270)
    at main (wasm-function[5]:46)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:126:20)
    at Module._compile (module.js:641:30)
    at Object.Module._extensions..js (module.js:652:10)
    at Module.load (module.js:560:32)
    at tryModuleLoad (module.js:503:12)
    at Function.Module._load (module.js:495:3)
    at Function.Module.runMain (module.js:682:10)
    at startup (bootstrap_node.js:191:16)

------------------------------------------

failures:
    [ui] ui/async-await/async-fn-size-moved-locals.rs
