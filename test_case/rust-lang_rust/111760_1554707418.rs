plain
[RUSTC-TIMING] coretests test:true 56.547
    Finished release [optimized] target(s) in 57.73s
     Running tests/lib.rs (obj/build/x86_64-unknown-linux-gnu/stage1-std/wasm32-unknown-unknown/release/deps/coretests-6c259f203458ed7b.wasm)
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[1735]:0x19008b)
    at rust_panic (<anonymous>:wasm-function[1718]:0x18fe95)
    at _ZN3std9panicking20rust_panic_with_hook17hb798ab2cbcfb6ba3E (<anonymous>:wasm-function[1717]:0x18fe87)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hb598452ca9597719E (<anonymous>:wasm-function[1702]:0x18ed48)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h16d97b514d01d699E (<anonymous>:wasm-function[1701]:0x18ecad)
    at rust_begin_unwind (<anonymous>:wasm-function[1712]:0x18fb17)
    at _ZN4core9panicking9panic_fmt17h7c90e7ba60ea945fE (<anonymous>:wasm-function[1808]:0x191dd8)
    at _ZN9coretests4iter8adapters11map_windows11check_drops17h0a4690f7ef288b23E (<anonymous>:wasm-function[860]:0xa6b2c)
    at _ZN4core3ops8function6FnOnce9call_once17ha2f6d7281640b5e4E (<anonymous>:wasm-function[900]:0xb33cd)
    at _ZN4test28__rust_begin_short_backtrace17h312d847273667169E (<anonymous>:wasm-function[1334]:0x15b303)
    at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h687d8dbf8038904cE (<anonymous>:wasm-function[1333]:0x15b2f6)
    at _ZN4test8run_test14run_test_inner17hf75e6699f1a5560cE (<anonymous>:wasm-function[1537]:0x1837ae)
    at _ZN4test8run_test17h10680a0667226300E (<anonymous>:wasm-function[1441]:0x16bec2)
    at _ZN4test7console17run_tests_console17hbd2b2316100f4794E (<anonymous>:wasm-function[1434]:0x167df4)
    at _ZN4test9test_main17h97c1afa22e19acf5E (<anonymous>:wasm-function[1535]:0x182a70)
    at _ZN4test16test_main_static17h4a2ded96bed224f0E (<anonymous>:wasm-function[1536]:0x182c5d)
    at _ZN9coretests4main17h52b8350c3bb86c19E (<anonymous>:wasm-function[1281]:0x156d9b)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h89c420c423ecde24E (<anonymous>:wasm-function[9]:0x25d6)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9b233e0216e07e05E (<anonymous>:wasm-function[10]:0x25f8)
    at _ZN3std2rt19lang_start_internal17hd23222e4fb683f80E (<anonymous>:wasm-function[1634]:0x18c3f8)
error: test failed, to rerun pass `-p core --test coretests`
