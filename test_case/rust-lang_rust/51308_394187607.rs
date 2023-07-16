plain
[00:45:13] ...........................................................................i........................
[00:45:18] ....................................................................................................
[00:45:23] ....................................................................................................
[00:45:29] ....................................................................................................
[00:45:34] ........i.................iiiiiiiii...................................................
[00:45:34] 
[00:45:34] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:23] ...........................................................................i........................
[00:46:28] ....................................................................................................
[00:46:32] ....................................................................................................
[00:46:38] ....................................................................................................
[00:46:42] ........i.................iiiiiiiii...................................................
[00:46:42] 
[00:46:42]  finished in 68.693
[00:46:42] travis_fold:end:test_ui_nll

---
[00:55:53] ....................................................................................................
[00:55:58] ....................................................................................................
[00:56:03] ....................................................................................................
[00:56:08] ....................................................................................................
[00:56:15] .......................................................F...FF.........................i.............
[00:56:26] ..............................................................................i.....................
[00:56:31] .......................i............................................................................
[00:56:35] ..............................................................................................i.....
[00:56:40] ....................................................................................................
---
[00:58:03] failures:
[00:58:03] 
[00:58:03] ---- [compile-fail] compile-fail/const-err-early.rs stdout ----
[00:58:03] 
[00:58:03] error: /checkout/src/test/compile-fail/const-err-early.rs:22: unexpected error: '22:1: 22:28: this constant cannot be used [const_err]'
[00:58:03] 
[00:58:03] error: 1 unexpected errors found, 0 expected errors not found
[00:58:03] status: exit code: 101
[00:58:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-err-early.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err-early/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err-early/auxiliary" "-A" "unused"
[00:58:03] unexpected errors (from JSON output): [
[00:58:03]     Error {
[00:58:03]         line_num: 22,
[00:58:03]         kind: Some(
[00:58:03]         ),
[00:58:03]         ),
[00:58:03]         msg: "22:1: 22:28: this constant cannot be used [const_err]"
[00:58:03] ]
[00:58:03] 
[00:58:03] thread '[compile-fail] compile-fail/const-err-early.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:58:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:03] 
[00:58:03] ---- [compile-fail] compile-fail/const-err3.rs stdout ----
[00:58:03] 
[00:58:03] error: /checkout/src/test/compile-fail/const-err3.rs:25: unexpected error: '25:14: 25:22: index out of bounds: the len is 1 but the index is 1 [const_err]'
[00:58:03] 
[00:58:03] error: 1 unexpected errors found, 0 expected errors not found
[00:58:03] status: exit code: 101
[00:58:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-err3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err3/auxiliary" "-A" "unused"
[00:58:03] unexpected errors (from JSON output): [
[00:58:03]     Error {
[00:58:03]         line_num: 25,
[00:58:03]         kind: Some(
[00:58:03]         ),
[00:58:03]         ),
[00:58:03]         msg: "25:14: 25:22: index out of bounds: the len is 1 but the index is 1 [const_err]"
[00:58:03] ]
[00:58:03] 
[00:58:03] thread '[compile-fail] compile-fail/const-err3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:58:03] 
[00:58:03] 
[00:58:03] ---- [compile-fail] compile-fail/const-err2.rs stdout ----
[00:58:03] 
[00:58:03] error: /checkout/src/test/compile-fail/const-err2.rs:33: unexpected error: '33:14: 33:22: index out of bounds: the len is 1 but the index is 1 [const_err]'
[00:58:03] 
[00:58:03] error: 1 unexpected errors found, 0 expected errors not found
[00:58:03] status: exit code: 101
[00:58:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-err2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-err2/auxiliary" "-A" "unused"
[00:58:03] unexpected errors (from JSON output): [
[00:58:03]     Error {
[00:58:03]         line_num: 33,
[00:58:03]         kind: Some(
[00:58:03]         ),
[00:58:03]         ),
[00:58:03]         msg: "33:14: 33:22: index out of bounds: the len is 1 but the index is 1 [const_err]"
[00:58:03] ]
[00:58:03] 
[00:58:03] thread '[compile-fail] compile-fail/const-err2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:58:03] 
---
[00:58:03] 
[00:58:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:58:03] 
[00:58:03] 
[00:58:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:03] 
[00:58:03] 
[00:58:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:03] Build completed unsuccessfully in 0:15:00
[00:58:03] Build completed unsuccessfully in 0:15:00
[00:58:03] Makefile:58: recipe for target 'check' failed
[00:58:03] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03f3ee51
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
