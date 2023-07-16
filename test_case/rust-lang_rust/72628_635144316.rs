
[RUSTC-TIMING] coretests test:true 64.186
    Finished release [optimized] target(s) in 1m 09s
     Running build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/coretests-b7cfeee46333c4a6.wasm
RuntimeError: unreachable
    at __rust_start_panic (wasm-function[1775]:1)
    at rust_panic (wasm-function[1763]:39)
    at _ZN3std9panicking20rust_panic_with_hook17hb4e2fb8af4804e97E (wasm-function[1758]:279)
    at _ZN3std9panicking11begin_panic17hb9f43a2c430997d1E (wasm-function[1107]:55)
    at _ZN4core5array75_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$_$u5d$$GT$7default17h75356a07fe8c5952E (wasm-function[410]:287)
    at _ZN4core3ops8function6FnOnce9call_once17h16c8e7c1f7d576cdE (wasm-function[409]:25)
    at _ZN4test28__rust_begin_short_backtrace17h804c565a83fc8136E (wasm-function[1401]:3)
    at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd4fcf7a756fe71aeE (wasm-function[1400]:6)
    at _ZN4test8run_test14run_test_inner17h4c09558679e02456E (wasm-function[1517]:2059)
    at _ZN4test8run_test17h0760c0ff80bcda5cE (wasm-function[1514]:812)
    at _ZN4test9run_tests17ha8a597da42e5b48eE (wasm-function[1468]:14251)
    at _ZN4test7console17run_tests_console17h90e9ea459accc65eE (wasm-function[1466]:1038)
    at _ZN4test9test_main17hff62743d0c9cc2e9E (wasm-function[1510]:332)
    at _ZN4test16test_main_static17hb90aede56671ffa7E (wasm-function[1512]:356)
    at _ZN9coretests4main17hfd760802efbf9d18E (wasm-function[991]:10)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hb3e0ac0d8e7d9f29E (wasm-function[325]:25)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4faef310d2c1b955E (wasm-function[1744]:8)
    at _ZN3std2rt19lang_start_internal17h08ff942ca8d003dfE (wasm-function[1764]:229)
    at main (wasm-function[992]:46)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
error: test failed, to rerun pass '-p core --test coretests'
