
---- [ui] ui/eprint-on-tls-drop.rs stdout ----
stderr:
------------------------------------------
RuntimeError: unreachable
    at __rust_start_panic (wasm-function[122]:1)
    at rust_panic (wasm-function[43]:39)
    at _ZN3std9panicking20rust_panic_with_hook17he47b91edb8082eabE (wasm-function[38]:279)
    at rust_begin_unwind (wasm-function[37]:84)
    at _ZN4core9panicking9panic_fmt17he18e83d766016c7aE (wasm-function[178]:58)
    at _ZN4core6result13unwrap_failed17h9fc1fc3320bec770E (wasm-function[149]:139)
    at _ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h708bda3f34d3945bE (wasm-function[2]:67)
    at _ZN18eprint_on_tls_drop4main17he6677df84c8c9a17E (wasm-function[3]:106)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcb9c038157d3e6aeE (wasm-function[5]:25)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h1d562f1bc2cf93c6E (wasm-function[114]:8)
    at _ZN3std9panicking3try7do_call17h7e855ca6cb87c0f9E.llvm.9727205892680777974 (wasm-function[36]:20)
    at __rust_maybe_catch_panic (wasm-function[121]:5)
    at _ZN3std2rt19lang_start_internal17h32edf9b2fff1c357E (wasm-function[66]:125)
    at main (wasm-function[4]:46)
    at Object.<anonymous> (/Users/eric/Proj/rust/rust/src/etc/wasm32-shim.js:20:20)
    at Module._compile (internal/modules/cjs/loader.js:868:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:879:10)
    at Module.load (internal/modules/cjs/loader.js:731:32)
    at Function.Module._load (internal/modules/cjs/loader.js:644:12)
    at Function.Module.runMain (internal/modules/cjs/loader.js:931:10)
