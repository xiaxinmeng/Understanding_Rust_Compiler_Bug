plain
[01:11:10] [RUSTC-TIMING] coretests test:true 94.557
[01:11:10]     Finished release [optimized] target(s) in 1m 42s
[01:11:10]      Running build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/coretests-7f0cbf4f70a8e2de.wasm
[01:11:12] RuntimeError: unreachable
[01:11:12]     at __rust_start_panic (wasm-function[1124]:45)
[01:11:12]     at rust_panic (wasm-function[1111]:39)
[01:11:12]     at _ZN3std9panicking20rust_panic_with_hook17hc2c13f1d99b58d10E (wasm-function[1106]:346)
[01:11:12]     at _ZN3std9panicking18continue_panic_fmt17h80bcb903aad73512E (wasm-function[1105]:151)
[01:11:12]     at _ZN3std9panicking15begin_panic_fmt17hd9bf6ce25f316536E (wasm-function[982]:108)
[01:11:12]     at _ZN39_$LT$R$u20$as$u20$rand..FromEntropy$GT$12from_entropy28_$u7b$$u7b$closure$u7d$$u7d$17h854d3f2a84855e91E (wasm-function[9]:84)
[01:11:12]     at _ZN9coretests5slice18partition_at_index17h13b4889758b2d0b5E (wasm-function[545]:10497)
[01:11:12]     at _ZN4core3ops8function6FnOnce9call_once17hcfce963d06aeb0d6E (wasm-function[544]:21)
[01:11:12]     at _ZN4test28__rust_begin_short_backtrace17h494b34b0b35103f4E (wasm-function[783]:3)
[01:11:12]     at _ZN50_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$8call_box17h695e58f6fc648d60E (wasm-function[782]:6)
[01:11:12]     at _ZN3std9panicking3try7do_call17hed02831d2b67832cE (wasm-function[709]:14)
[01:11:12]     at __rust_maybe_catch_panic (wasm-function[1123]:5)
[01:11:12]     at _ZN4test8run_test14run_test_inner17h06ba453083ee074cE (wasm-function[864]:801)
[01:11:12]     at _ZN4test8run_test17hbea81826c4538903E (wasm-function[861]:2119)
[01:11:12]     at _ZN4test9run_tests17hfda02b999bc6fba5E (wasm-function[856]:4862)
[01:11:12]     at _ZN4test17run_tests_console17hc45fe3f2218d5807E (wasm-function[850]:993)
[01:11:12]     at _ZN4test9test_main17hc6e84b95e2982be1E (wasm-function[847]:433)
[01:11:12]     at _ZN4test16test_main_static17hfcd7d3e85b15a62bE (wasm-function[851]:1747)
[01:11:12]     at _ZN9coretests4main17h442538944079574cE (wasm-function[653]:10)
[01:11:12]     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hab1980bbadd9a85aE (wasm-function[10]:25)
[01:11:12] 
[01:11:12] 
[01:11:12] 
[01:11:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
[01:11:12] 
[01:11:12] 
[01:11:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:11:12] Build completed unsuccessfully in 1:08:12
---
travis_time:end:050ce4ba:start=1548274625825578526,finish=1548274625845108104,duration=19529578
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0355f0ca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f8b7fb4
travis_time:start:0f8b7fb4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:047216b8
$ dmesg | grep -i kill
