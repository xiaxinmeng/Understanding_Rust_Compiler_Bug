plain
[00:52:18] ..........................................................................i.........................
[00:52:23] ....................................................................................................
[00:52:29] ....................................................................................................
[00:52:36] ....................................................................................................
[00:52:40] ......i.................iiiiiiiii...................................................
[00:52:40] 
[00:52:40] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:53:31] ..........................................................................i.........................
[00:53:35] ....................................................................................................
[00:53:40] ....................................................................................................
[00:53:46] ....................................................................................................
[00:53:50] ......i.................iiiiiiiii...................................................
[00:53:50] 
[00:53:50]  finished in 70.145
[00:53:50] travis_fold:end:test_ui_nll

---
[01:04:01] ..............................................................................................i.....
[01:04:05] ....................................................................................................
[01:04:12] ....................................................................................................
[01:04:18] ....................................................................................................
[01:04:25] ..................................................F.................................................
[01:04:40] ....................................................................................................
[01:04:46] ...i................................................................................................
[01:04:53] ..i..ii.............................................................................................
[01:04:53] ..i..ii.............................................................................................
[01:04:59] ........................................................................FFF.........................
[01:05:09] ..............................................................................i.....................
[01:05:15] ........................i...........................................................................
[01:05:21] ....................................................................................................
[01:05:26] ....................................................................................................
[01:05:26] ....................................................................................................
[01:05:30] ....................................................................................................
[01:05:31] ....................
[01:05:31] failures:
[01:05:31] 
[01:05:31] ---- [compile-fail] compile-fail/issue-39616.rs stdout ----
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/issue-39616.rs:11: unexpected error: '11:16: 11:17: expected one of `)`, `,`, `->`, `where`, or `{`, found `]`'
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/issue-39616.rs:11: expected error not found: expected one of `->`, `where`, or `{`, found `]`
[01:05:31] 
[01:05:31] error: 1 unexpected errors found, 1 expected errors not found
[01:05:31] status: exit code: 101
[01:05:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-39616.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-39616/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-39616/auxiliary" "-A" "unused"
[01:05:31] unexpected errors (from JSON output): [
[01:05:31]     Error {
[01:05:31]         line_num: 11,
[01:05:31]         kind: Some(
[01:05:31]         ),
[01:05:31]         ),
[01:05:31]         msg: "11:16: 11:17: expected one of `)`, `,`, `->`, `where`, or `{`, found `]`"
[01:05:31] ]
[01:05:31] 
[01:05:31] not found errors (from test file): [
[01:05:31]     Error {
[01:05:31]     Error {
[01:05:31]         line_num: 11,
[01:05:31]         kind: Some(
[01:05:31]             Error
[01:05:31]         ),
[01:05:31]         msg: "expected one of `->`, `where`, or `{`, found `]`"
[01:05:31] ]
[01:05:31] 
[01:05:31] thread '[compile-fail] compile-fail/issue-39616.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:05:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:31] 
[01:05:31] ---- [compile-fail] compile-fail/privacy/restricted/tuple-struct-fields/test.rs stdout ----
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test.rs:14: unexpected error: '14:26: 14:27: expected one of `)` or `,`, found `(`'
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test.rs:14: expected error not found: expected `,`, found `(`
[01:05:31] 
[01:05:31] error: 1 unexpected errors found, 1 expected errors not found
[01:05:31] status: exit code: 101
[01:05:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/privacy/restricted/tuple-struct-fields/test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/privacy/restricted/tuple-struct-fields/test/auxiliary" "-A" "unused"
[01:05:31] unexpected errors (from JSON output): [
[01:05:31]     Error {
[01:05:31]         line_num: 14,
[01:05:31]         kind: Some(
[01:05:31]         ),
[01:05:31]         ),
[01:05:31]         msg: "14:26: 14:27: expected one of `)` or `,`, found `(`"
[01:05:31] ]
[01:05:31] 
[01:05:31] not found errors (from test file): [
[01:05:31]     Error {
[01:05:31]     Error {
[01:05:31]         line_num: 14,
[01:05:31]         kind: Some(
[01:05:31]             Error
[01:05:31]         ),
[01:05:31]         msg: "expected `,`, found `(`"
[01:05:31] ]
[01:05:31] 
[01:05:31] thread '[compile-fail] compile-fail/privacy/restricted/tuple-struct-fields/test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:05:31] 
[01:05:31] 
[01:05:31] ---- [compile-fail] compile-fail/privacy/restricted/tuple-struct-fields/test2.rs stdout ----
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test2.rs:15: unexpected error: '15:26: 15:27: expected one of `)` or `,`, found `(`'
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test2.rs:15: expected error not found: expected `,`, found `(`
[01:05:31] 
[01:05:31] error: 1 unexpected errors found, 1 expected errors not found
[01:05:31] status: exit code: 101
[01:05:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/privacy/restricted/tuple-struct-fields/test2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/privacy/restricted/tuple-struct-fields/test2/auxiliary" "-A" "unused"
[01:05:31] unexpected errors (from JSON output): [
[01:05:31]     Error {
[01:05:31]         line_num: 15,
[01:05:31]         kind: Some(
[01:05:31]         ),
[01:05:31]         ),
[01:05:31]         msg: "15:26: 15:27: expected one of `)` or `,`, found `(`"
[01:05:31] ]
[01:05:31] 
[01:05:31] not found errors (from test file): [
[01:05:31]     Error {
[01:05:31]     Error {
[01:05:31]         line_num: 15,
[01:05:31]         kind: Some(
[01:05:31]             Error
[01:05:31]         ),
[01:05:31]         msg: "expected `,`, found `(`"
[01:05:31] ]
[01:05:31] 
[01:05:31] thread '[compile-fail] compile-fail/privacy/restricted/tuple-struct-fields/test2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:05:31] 
[01:05:31] 
[01:05:31] ---- [compile-fail] compile-fail/privacy/restricted/tuple-struct-fields/test3.rs stdout ----
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test3.rs:15: unexpected error: '15:27: 15:28: expected one of `)` or `,`, found `(`'
[01:05:31] 
[01:05:31] error: /checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test3.rs:15: expected error not found: expected `,`, found `(`
[01:05:31] 
[01:05:31] error: 1 unexpected errors found, 1 expected errors not found
[01:05:31] status: exit code: 101
[01:05:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/privacy/restricted/tuple-struct-fields/test3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/privacy/restricted/tuple-struct-fields/test3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/privacy/restricted/tuple-struct-fields/test3/auxiliary" "-A" "unused"
[01:05:31] unexpected errors (from JSON output): [
[01:05:31]     Error {
[01:05:31]         line_num: 15,
[01:05:31]         kind: Some(
[01:05:31]         ),
[01:05:31]         ),
[01:05:31]         msg: "15:27: 15:28: expected one of `)` or `,`, found `(`"
[01:05:31] ]
[01:05:31] 
[01:05:31] not found errors (from test file): [
[01:05:31]     Error {
[01:05:31]     Error {
[01:05:31]         line_num: 15,
[01:05:31]         kind: Some(
[01:05:31]             Error
[01:05:31]         ),
[01:05:31]         msg: "expected `,`, found `(`"
[01:05:31] ]
[01:05:31] 
[01:05:31] thread '[compile-fail] compile-fail/privacy/restricted/tuple-struct-fields/test3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:05:31] 
---
[01:05:31] 
[01:05:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:05:31] 
[01:05:31] 
[01:05:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:31] 
[01:05:31] 
[01:05:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:31] Build completed unsuccessfully in 0:15:42
[01:05:31] Build completed unsuccessfully in 0:15:42
[01:05:31] make: *** [check] Error 1
[01:05:31] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05049f38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
