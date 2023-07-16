plain
[00:47:31] ..........................................................................i.........................
[00:47:36] ....................................................................................................
[00:47:42] ....................................................................................................
[00:47:49] ....................................................................................................
[00:47:53] ......i.................iiiiiiiii...................................................
[00:47:53] 
[00:47:53] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:47] ..........................................................................i.........................
[00:48:52] ....................................................................................................
[00:48:57] ....................................................................................................
[00:49:03] ....................................................................................................
[00:49:08] ......i..................iiiiiiiii..................................................
[00:49:08] 
[00:49:08]  finished in 74.521
[00:49:08] travis_fold:end:test_ui_nll

---

[01:07:33] travis_fold:start:test_run-fail-fulldeps
travis_time:start:test_run-fail-fulldeps
Check compiletest suite=run-fail-fulldeps mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:33] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:945:5
[01:07:33] 
[01:07:33] 
[01:07:33] 
[01:07:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:33] 
[01:07:33] 
[01:07:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:33] Build completed unsuccessfully in 0:22:28
[01:07:33] Build completed unsuccessfully in 0:22:28
[01:07:33] Makefile:58: recipe for target 'check' failed
[01:07:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0430fad9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
