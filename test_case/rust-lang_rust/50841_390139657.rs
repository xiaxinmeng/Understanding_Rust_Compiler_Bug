plain
[00:57:16] .................................i..................................................................
[00:57:37] .............................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:57:46] .......................................
[00:58:01] ....................................................................................................
[00:58:48] .ii..............................................................i..................................
[00:59:06] ..................i.ii...............................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:59:47] ..........................iiiiiii...................................................................
[01:00:04] ....................................................................................................
[01:00:19] ....................................................................................................
[01:00:36] .................................................................................................
---
Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:53] 
[01:02:53] running 143 tests
[01:03:03] ....................................................................................................
[01:03:08] ...........F......i........................
[01:03:08] 
[01:03:08] ---- [run-fail] run-fail/promoted_overflow.rs stdout ----
[01:03:08] 
[01:03:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:03:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:03:08] error: Error: expected failure status (Some(101)) but received status Some(0).
[01:03:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_overflow/a"
[01:03:08] stdout:
[01:03:08] ------------------------------------------
[01:03:08] 
---
[01:03:08] test result: FAILED. 141 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:03:08] 
[01:03:08] 
[01:03:08] 
[01:03:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:08] 
[01:03:08] 
[01:03:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:08] Build completed unsuccessfully in 0:15:04
[01:03:08] Build completed unsuccessfully in 0:15:04
[01:03:08] Makefile:58: recipe for target 'check' failed
[01:03:08] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25e66edf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
