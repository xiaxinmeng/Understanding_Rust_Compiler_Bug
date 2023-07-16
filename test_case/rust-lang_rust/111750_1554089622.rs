plain
[RUSTC-TIMING] coretests test:true 42.842
    Finished release [optimized] target(s) in 43.93s
     Running tests/lib.rs (obj/build/x86_64-unknown-linux-gnu/stage1-std/wasm32-unknown-unknown/release/deps/coretests-6c259f203458ed7b.wasm)
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[1735]:0x19008e)
    at rust_panic (<anonymous>:wasm-function[1718]:0x18fe98)
    at _ZN3std9panicking20rust_panic_with_hook17hb9038416d9a2b97eE (<anonymous>:wasm-function[1717]:0x18fe8a)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hd90da3d565df938dE (<anonymous>:wasm-function[1702]:0x18ed4b)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h2528644a8b8b1ca8E (<anonymous>:wasm-function[1701]:0x18ecb0)
    at rust_begin_unwind (<anonymous>:wasm-function[1712]:0x18fb1a)
    at _ZN4core9panicking9panic_fmt17h5a53469b5de8b928E (<anonymous>:wasm-function[1808]:0x191ddb)
    at _ZN9coretests4iter8adapters11map_windows11check_drops17hd29bf682b72b1f43E (<anonymous>:wasm-function[415]:0x39445)
    at _ZN4core3ops8function6FnOnce9call_once17hbc937db7ce74843cE (<anonymous>:wasm-function[956]:0xe58a1)
    at _ZN4test28__rust_begin_short_backtrace17hbed63b6082c0a189E (<anonymous>:wasm-function[1327]:0x15aa85)
    at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h165fcc6ca8039a52E (<anonymous>:wasm-function[1326]:0x15aa78)
    at _ZN4test8run_test14run_test_inner17h711b907f941ab7afE (<anonymous>:wasm-function[1537]:0x1837b0)
    at _ZN4test8run_test17hfebebce077fffbfeE (<anonymous>:wasm-function[1441]:0x16bec4)
    at _ZN4test7console17run_tests_console17h79937b90534d8017E (<anonymous>:wasm-function[1434]:0x167df6)
    at _ZN4test9test_main17h544617e592227b77E (<anonymous>:wasm-function[1535]:0x182a72)
    at _ZN4test16test_main_static17hbc98b15c350ff64dE (<anonymous>:wasm-function[1536]:0x182c5f)
    at _ZN9coretests4main17hdd946f3578c18ee5E (<anonymous>:wasm-function[1281]:0x156d9c)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h3ad8d3044a0fa301E (<anonymous>:wasm-function[9]:0x25d8)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h361d9415fb921c66E (<anonymous>:wasm-function[10]:0x25fa)
    at _ZN3std2rt19lang_start_internal17h2f6827eb9af84dd6E (<anonymous>:wasm-function[1634]:0x18c3fb)
error: test failed, to rerun pass `-p core --test coretests`
