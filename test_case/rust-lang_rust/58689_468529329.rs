
[01:19:52] [0m[0m[1m[32m     Running[0m build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/coretests-a4433084340ef73b.wasm
[01:19:53] RuntimeError: unreachable
[01:19:53]     at __rust_start_panic (wasm-function[1095]:45)
[01:19:53]     at rust_panic (wasm-function[1082]:39)
[01:19:53]     at _ZN3std9panicking20rust_panic_with_hook17hd0681d90b51f9001E (wasm-function[1077]:346)
[01:19:53]     at _ZN3std9panicking18continue_panic_fmt17heb2c3399be9c1647E (wasm-function[1076]:151)
[01:19:53]     at _ZN3std9panicking15begin_panic_fmt17heae8ec5c6936ff86E (wasm-function[955]:108)
[01:19:53]     at _ZN4core3ops8function6FnOnce9call_once17h8654601badcab08eE (wasm-function[418]:500)
[01:19:53]     at _ZN4test28__rust_begin_short_backtrace17h5acac12ba406c08cE (wasm-function[753]:3)
[01:19:53]     at _ZN50_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$8call_box17h3970079b2a2e8325E (wasm-function[752]:6)
[01:19:53]     at _ZN3std9panicking3try7do_call17h2a54e10a8ef5b9a4E (wasm-function[679]:14)
[01:19:53]     at __rust_maybe_catch_panic (wasm-function[1094]:5)
[01:19:53]     at _ZN4test8run_test14run_test_inner17ha9d0c3c920b17a93E (wasm-function[833]:801)
[01:19:53]     at _ZN4test8run_test17h9570bc596a31e164E (wasm-function[830]:2119)
[01:19:53]     at _ZN4test9run_tests17hac4301576bb3c3acE (wasm-function[825]:3408)
[01:19:53]     at _ZN4test17run_tests_console17hddadae4b0dffc12fE (wasm-function[819]:993)
[01:19:53]     at _ZN4test9test_main17h343b975aae7c6b90E (wasm-function[816]:433)
[01:19:53]     at _ZN4test16test_main_static17hfd242c8df31b861aE (wasm-function[820]:1746)
[01:19:53]     at _ZN9coretests4main17h59dcd2e523abb771E (wasm-function[642]:10)
[01:19:53]     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h409059acbb92528aE (wasm-function[10]:25)
[01:19:53]     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcfd224ff90044727E (wasm-function[1060]:8)
[01:19:53]     at _ZN3std9panicking3try7do_call17h526f0906eecc5923E (wasm-function[1073]:20)
[01:19:53] [0m[0m[1m[31merror:[0m test failed, to rerun pass '--test coretests'
[01:19:53] 
[01:19:53] 
[01:19:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
[01:19:53] expected success, got: exit code: 101
