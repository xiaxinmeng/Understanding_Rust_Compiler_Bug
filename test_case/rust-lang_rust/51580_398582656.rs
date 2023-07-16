plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:56] 
[00:57:56] running 94 tests
[00:59:42] ...............................................F................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:02:10] failures:
[01:02:10] 
[01:02:10] ---- [run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs stdout ----
[01:02:10] 
[01:02:10] 
[01:02:10] error: compilation failed!
[01:02:10] status: exit code: 101
[01:02:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/auxiliary"
[01:02:10] ------------------------------------------
[01:02:10] 
[01:02:10] ------------------------------------------
[01:02:10] stderr:
[01:02:10] stderr:
[01:02:10] ------------------------------------------
[01:02:10] error[E0061]: this function takes 6 parameters but 5 parameters were supplied
[01:02:10]    --> /checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs:126:25
[01:02:10]     |
[01:02:10] 126 | /                         ExprKind::Closure(CaptureBy::Value,
[01:02:10] 127 | |                                           Movability::Movable,
[01:02:10] 128 | |                                           decl.clone(),
[01:02:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:02:10] 129 | |                                           e,
[01:02:10] 130 | |                                           DUMMY_SP)));
[01:02:10] 
[01:02:10] error: aborting due to previous error
[01:02:10] 
[01:02:10] For more information about this error, try `rustc --explain E0061`.
---
[01:02:10] test result: FAILED. 93 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:02:10] 
[01:02:10] 
[01:02:10] 
[01:02:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:10] 
[01:02:10] 
[01:02:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:10] Build completed unsuccessfully in 0:20:11
[01:02:10] Build completed unsuccessfully in 0:20:11
[01:02:10] Makefile:58: recipe for target 'check' failed
[01:02:10] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11975f09
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
