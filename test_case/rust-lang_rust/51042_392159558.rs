plain
[01:17:28] ..............................................................................................i.....
[01:17:32] ....................................................................................................
[01:17:39] ....................................................................................................
[01:17:45] ....................................................................................................
[01:17:52] ................F...................................................................................
[01:18:08] ....................................................................................................
[01:18:14] ...i................................................................................................
[01:18:21] ..i..ii.............................................................................................
[01:18:30] ....................................................................................................
---
[01:19:10] failures:
[01:19:10] 
[01:19:10] ---- [compile-fail] compile-fail/issue-36839.rs stdout ----
[01:19:10] 
[01:19:10] error: /checkout/src/test/compile-fail/issue-36839.rs:24: unexpected error: '24:5: 26:6: the trait bound `(): Foo` is not satisfied [E0277]'
[01:19:10] 
[01:19:10] error: /checkout/src/test/compile-fail/issue-36839.rs:30: expected error not found: compilation successful
[01:19:10] 
[01:19:10] error: 1 unexpected errors found, 1 expected errors not found
[01:19:10] status: exit code: 101
[01:19:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-36839.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-36839/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-36839/auxiliary" "-A" "unused"
[01:19:10] unexpected errors (from JSON output): [
[01:19:10]     Error {
[01:19:10]         line_num: 24,
[01:19:10]         kind: Some(
[01:19:10]         ),
[01:19:10]         ),
[01:19:10]         msg: "24:5: 26:6: the trait bound `(): Foo` is not satisfied [E0277]"
[01:19:10] ]
[01:19:10] 
[01:19:10] not found errors (from test file): [
[01:19:10]     Error {
[01:19:10]     Error {
[01:19:10]         line_num: 30,
[01:19:10]         kind: Some(
[01:19:10]             Error
[01:19:10]         ),
[01:19:10]         msg: "compilation successful"
[01:19:10] ]
[01:19:10] 
[01:19:10] thread '[compile-fail] compile-fail/issue-36839.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:19:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:19:10] 
[01:19:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:19:10] 
[01:19:10] 
[01:19:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:10] 
[01:19:10] 
[01:19:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:10] Build completed unsuccessfully in 0:20:03
[01:19:10] Build completed unsuccessfully in 0:20:03
[01:19:10] Makefile:58: recipe for target 'check' failed
[01:19:10] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f7d4b79
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
