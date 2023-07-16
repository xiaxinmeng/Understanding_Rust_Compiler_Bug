
---- [ui] ui/tls.rs stdout ----

error: test run failed!
status: exit code: 101
command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tls/a.wasm"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
RuntimeError: unreachable
    at __rust_start_panic (wasm-function[89]:1)
    at rust_panic (wasm-function[83]:39)
    at _ZN3std9panicking20rust_panic_with_hook17h4caac20c72d75dc1E (wasm-function[78]:279)
    at rust_begin_unwind (wasm-function[77]:90)
    at _ZN4core9panicking9panic_fmt17h2a7f8c32bf7391a0E (wasm-function[108]:58)
    at _ZN4core6option18expect_none_failed17h300c1b4bc80874faE (wasm-function[118]:139)
    at _ZN3std6thread5spawn17hd38fc9ede2951672E (wasm-function[4]:407)
    at _ZN3tls4main17hfb802c933ee14077E (wasm-function[8]:25)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7dab73fdfb41bcd3E (wasm-function[10]:25)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hee2c79df7781245bE (wasm-function[67]:8)
    at _ZN3std2rt19lang_start_internal17h5bcb99e6d909e866E (wasm-function[84]:229)
    at main (wasm-function[9]:46)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (module.js:641:30)
    at Object.Module._extensions..js (module.js:652:10)
    at Module.load (module.js:560:32)
    at tryModuleLoad (module.js:503:12)
    at Function.Module._load (module.js:495:3)
    at Function.Module.runMain (module.js:682:10)
    at startup (bootstrap_node.js:191:16)

------------------------------------------
