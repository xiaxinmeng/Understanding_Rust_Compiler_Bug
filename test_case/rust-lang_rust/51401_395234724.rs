plain
[00:42:55] ..i..............................................................................i..................
[00:42:59] ....................................................................................................
[00:43:05] ....................................................................................................
[00:43:11] ....................................................................................................
[00:43:15] ...............i.................iiiiiiiii...................................................
[00:43:15] 
[00:43:15] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:04] ..i..............................................................................i..................
[00:44:09] ....................................................................................................
[00:44:13] ....................................................................................................
[00:44:19] ....................................................................................................
[00:44:23] ...............i.................iiiiiiiii...................................................
[00:44:23] 
[00:44:23]  finished in 67.963
[00:44:23] travis_fold:end:test_ui_nll

---
[00:53:53] ....................................................................................................
[00:53:58] ....................................................................................................
[00:54:04] ....................................................................................................
[00:54:10] ....................................................................................................
[00:54:16] ......F.............................................................................................
[00:54:28] ....i...............................................................................................
[00:54:34] ...i..ii............................................................................................
[00:54:41] ....................................................................................................
[00:54:47] ....................................................................................................
---
[00:55:10] failures:
[00:55:10] 
[00:55:10] ---- [compile-fail] compile-fail/issue-43988.rs stdout ----
[00:55:10] 
[00:55:10] error: /checkout/src/test/compile-fail/issue-43988.rs:34: unexpected warning: '34:5: 34:12: `repr` attribute must have a hint [bad_repr]'
[00:55:10] 
[00:55:10] error: /checkout/src/test/compile-fail/issue-43988.rs:45: unexpected warning: '45:14: 45:21: `repr` attribute must have a hint [bad_repr]'
[00:55:10] 
[00:55:10] error: 2 unexpected errors found, 0 expected errors not found
[00:55:10] status: exit code: 101
[00:55:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-43988.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43988/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43988/auxiliary" "-A" "unused"
[00:55:10] unexpected errors (from JSON output): [
[00:55:10]     Error {
[00:55:10]         line_num: 34,
[00:55:10]         kind: Some(
[00:55:10]         ),
[00:55:10]         ),
[00:55:10]         msg: "34:5: 34:12: `repr` attribute must have a hint [bad_repr]"
[00:55:10]     Error {
[00:55:10]         line_num: 45,
[00:55:10]         kind: Some(
[00:55:10]             Warning
[00:55:10]             Warning
[00:55:10]         ),
[00:55:10]         msg: "45:14: 45:21: `repr` attribute must have a hint [bad_repr]"
[00:55:10] ]
[00:55:10] 
[00:55:10] thread '[compile-fail] compile-fail/issue-43988.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:55:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:55:10] 
[00:55:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:10] 
[00:55:10] 
[00:55:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:10] 
[00:55:10] 
[00:55:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:10] Build completed unsuccessfully in 0:14:24
[00:55:10] Build completed unsuccessfully in 0:14:24
[00:55:10] Makefile:58: recipe for target 'check' failed
[00:55:10] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21f81300
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
