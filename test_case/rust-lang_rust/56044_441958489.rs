plain
[01:05:17] 
[01:05:17] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:05:17] error: test run failed!
[01:05:17] status: exit code: 101
[01:05:17] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/fn-arg-incomplete-pattern-drop-order/a.wasm"
[01:05:17] ------------------------------------------
[01:05:17] 
[01:05:17] ------------------------------------------
[01:05:17] stderr:
[01:05:17] stderr:
[01:05:17] ------------------------------------------
[01:05:17] RuntimeError: unreachable
[01:05:17]     at __rust_start_panic (wasm-function[82]:1)
[01:05:17]     at rust_panic (wasm-function[78]:39)
[01:05:17]     at _ZN3std9panicking20rust_panic_with_hook17hefa20f9e2d7144fcE (wasm-function[73]:346)
[01:05:17]     at _ZN3std9panicking11begin_panic17ha07741734e150442E (wasm-function[11]:49)
[01:05:17]     at _ZN36fn_arg_incomplete_pattern_drop_order3foo17hb9cbcbcdc62d43caE (wasm-function[2]:607)
[01:05:17]     at _ZN3std9panicking3try7do_call17hbbbd02620233e1d7E.llvm.12228513419448781796 (wasm-function[12]:348)
[01:05:17]     at __rust_maybe_catch_panic (wasm-function[81]:5)
[01:05:17]     at _ZN36fn_arg_incomplete_pattern_drop_order4main17h78f606c69bb2c1c8E (wasm-function[3]:295)
[01:05:17]     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h898ef31c452dd768E (wasm-function[6]:25)
[01:05:17]     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd27616b21d0538aaE (wasm-function[62]:8)
[01:05:17]     at _ZN3std9panicking3try7do_call17hfb625b14a19c314fE (wasm-function[70]:20)
[01:05:17]     at __rust_maybe_catch_panic (wasm-function[81]:5)
[01:05:17]     at _ZN3std2rt19lang_start_internal17h89e1dbf50853ba79E (wasm-function[79]:270)
[01:05:17]     at _ZN3std2rt10lang_start17hbfa996f5e8ab0c4dE (wasm-function[5]:42)
[01:05:17]     at main (wasm-function[4]:11)
[01:05:17]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[01:05:17]     at Module._compile (module.js:641:30)
[01:05:17]     at Object.Module._extensions..js (module.js:652:10)
[01:05:17]     at Module.load (module.js:560:32)
[01:05:17]     at tryModuleLoad (module.js:503:12)
[01:05:17] ------------------------------------------
[01:05:17] 
[01:05:17] thread '[run-pass] run-pass/binding/fn-arg-incomplete-pattern-drop-order.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:05:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:05:17] test result: FAILED. 2636 passed; 1 failed; 251 ignored; 0 measured; 0 filtered out
[01:05:17] 
[01:05:17] 
[01:05:17] 
[01:05:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-pass" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:17] 
[01:05:17] 
[01:05:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:05:17] Build completed unsuccessfully in 1:02:53
---
travis_time:end:0b8a2e24:start=1543304505070614849,finish=1543304505077540370,duration=6925521
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02cf9286
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07de15a9
travis_time:start:07de15a9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:258e1492
$ dmesg | grep -i kill
