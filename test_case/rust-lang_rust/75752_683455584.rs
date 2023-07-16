
---- [ui] ui/issues/issue-16597-empty.rs stdout ----

error: test run failed!
status: exit code: 101
command: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/a.wasm"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
RuntimeError: unreachable
    at __rust_start_panic (wasm-function[440]:0x367e5)
    at rust_panic (wasm-function[429]:0x364b0)
    at _ZN3std9panicking20rust_panic_with_hook17hd145b35fc2e3a3cfE (wasm-function[424]:0x36193)
    at _ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17hb1773171f7f93b68E (wasm-function[412]:0x35e10)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hf7e9c7af0f61d223E (wasm-function[411]:0x35dd7)
    at _ZN3std9panicking11begin_panic17hffdce10487e9f2d5E (wasm-function[306]:0x3125a)
    at _ZN3std3sys4wasm4time7Instant3now17h0cd4438a2c9238d7E (wasm-function[404]:0x35bc2)
    at _ZN3std4time7Instant3now17hfc7bdb933c65609fE (wasm-function[398]:0x359b0)
    at _ZN4test7console17run_tests_console17h9236dc85784dfce6E (wasm-function[134]:0x10fd5)
    at _ZN4test9test_main17hf7027953a2d207bdE (wasm-function[182]:0x2198f)
    at _ZN4test16test_main_static17hd410e3e9c2ad1d88E (wasm-function[183]:0x22a9a)
    at _ZN17issue_16597_empty4main17hce00e52b5711c0baE (wasm-function[1]:0x545)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h527334a16a9115efE (wasm-function[0]:0x521)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hd43ebc70162051f2E.llvm.11756281066959917856 (wasm-function[3]:0x5ab)
    at _ZN3std2rt19lang_start_internal17h1326eb4f191ee6aaE (wasm-function[430]:0x3657a)
    at main (wasm-function[2]:0x57b)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (internal/modules/cjs/loader.js:1200:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:1220:10)
    at Module.load (internal/modules/cjs/loader.js:1049:32)

------------------------------------------
