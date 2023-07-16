plain
[00:51:42] ....................................................................i...............................
[00:51:47] ....................................................................................................
[00:51:53] ....................................................................................................
[00:52:00] .................................................................................................i..
[00:52:03] ...............iiiiiiiii...................................................
[00:52:03] 
[00:52:03] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:55] ....................................................................i...............................
[00:53:00] ....................................................................................................
[00:53:05] ....................................................................................................
[00:53:12] .................................................................................................i..
[00:53:15] ...............iiiiiiiii...................................................
[00:53:15] 
[00:53:15]  finished in 72.112
[00:53:15] travis_fold:end:test_ui_nll

---
[01:05:19] .ii.................................................................................................
[01:05:25] ....................................................................................................
[01:05:30] ....................................................................................................
[01:05:34] ..........................................................................i.........................
[01:05:40] ....................i.....................................F.........................................
[01:05:50] ....................................................................................................
[01:05:55] ....................................................................................................
[01:05:56] ................
[01:05:56] failures:
[01:05:56] failures:
[01:05:56] 
[01:05:56] ---- [compile-fail] compile-fail/static-array-across-crate.rs stdout ----
[01:05:56] 
[01:05:56] error: compile-fail test compiled successfully!
[01:05:56] status: exit code: 0
[01:05:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/static-array-across-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/auxiliary" "-A" "unused"
[01:05:56] ------------------------------------------
[01:05:56] 
[01:05:56] ------------------------------------------
[01:05:56] stderr:
---
[01:05:56] 
[01:05:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:05:56] 
[01:05:56] 
[01:05:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:56] 
[01:05:56] 
[01:05:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:56] Build completed unsuccessfully in 0:16:41
[01:05:56] Build completed unsuccessfully in 0:16:41
[01:05:56] make: *** [check] Error 1
[01:05:56] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fe2f779
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
