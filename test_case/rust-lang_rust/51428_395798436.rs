plain
[00:43:38] .i..............................................................................i...................
[00:43:43] ....................................................................................................
[00:43:48] ....................................................................................................
[00:43:55] ....................................................................................................
[00:43:59] .............i.................iiiiiiiii...................................................
[00:43:59] 
[00:43:59] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:50] .i..............................................................................i...................
[00:44:55] ....................................................................................................
[00:45:00] ....................................................................................................
[00:45:05] ....................................................................................................
[00:45:10] .............i.................iiiiiiiii...................................................
[00:45:10] 
[00:45:10]  finished in 70.736
[00:45:10] travis_fold:end:test_ui_nll

---
[00:54:24] ......................................................................................i.............
[00:54:30] ...............................................ii.iii...............................................
[00:54:34] ...............................................................................i....................
[00:54:39] ........................i...........................................................................
[00:54:44] ............................................................F..................................i....
[00:54:53] ....................................................................................................
[00:54:59] .F..................................................................................................
[00:55:04] ....................................................................................................
[00:55:10] ....................................................................................................
---
[00:56:05] failures:
[00:56:05] 
[00:56:05] ---- [compile-fail] compile-fail/issue-14227.rs stdout ----
[00:56:05] 
[00:56:05] error: /checkout/src/test/compile-fail/issue-14227.rs:16: unexpected error: '16:20: 16:26: could not evaluate static initializer [E0080]'
[00:56:05] 
[00:56:05] error: /checkout/src/test/compile-fail/issue-14227.rs:16: unexpected error: '16:1: 16:27: could not evaluate static initializer [E0080]'
[00:56:05] 
[00:56:05] error: /checkout/src/test/compile-fail/issue-14227.rs:16: expected error not found: constant evaluation error
[00:56:05] 
[00:56:05] error: 2 unexpected errors found, 1 expected errors not found
[00:56:05] status: exit code: 101
[00:56:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-14227.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/auxiliary" "-A" "unused"
[00:56:05] unexpected errors (from JSON output): [
[00:56:05]     Error {
[00:56:05]         line_num: 16,
[00:56:05]         kind: Some(
[00:56:05]         ),
[00:56:05]         ),
[00:56:05]         msg: "16:20: 16:26: could not evaluate static initializer [E0080]"
[00:56:05]     Error {
[00:56:05]         line_num: 16,
[00:56:05]         kind: Some(
[00:56:05]             Error
[00:56:05]             Error
[00:56:05]         ),
[00:56:05]         msg: "16:1: 16:27: could not evaluate static initializer [E0080]"
[00:56:05] ]
[00:56:05] 
[00:56:05] not found errors (from test file): [
[00:56:05]     Error {
[00:56:05]     Error {
[00:56:05]         line_num: 16,
[00:56:05]         kind: Some(
[00:56:05]             Error
[00:56:05]         ),
[00:56:05]         msg: "constant evaluation error"
[00:56:05] ]
[00:56:05] 
[00:56:05] thread '[compile-fail] compile-fail/issue-14227.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:05] 
[00:56:05] ---- [compile-fail] compile-fail/issue-28324.rs stdout ----
[00:56:05] 
[00:56:05] error: /checkout/src/test/compile-fail/issue-28324.rs:17: unexpected error: '17:23: 17:44: could not evaluate static initializer [E0080]'
[00:56:05] 
[00:56:05] error: /checkout/src/test/compile-fail/issue-28324.rs:17: unexpected error: '17:1: 17:45: could not evaluate static initializer [E0080]'
[00:56:05] 
[00:56:05] error: /checkout/src/test/compile-fail/issue-28324.rs:17: expected error not found: constant evaluation error
[00:56:05] 
[00:56:05] error: 2 unexpected errors found, 1 expected errors not found
[00:56:05] status: exit code: 101
[00:56:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-28324.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/auxiliary" "-A" "unused"
[00:56:05] unexpected errors (from JSON output): [
[00:56:05]     Error {
[00:56:05]         line_num: 17,
[00:56:05]         kind: Some(
[00:56:05]         ),
[00:56:05]         ),
[00:56:05]         msg: "17:23: 17:44: could not evaluate static initializer [E0080]"
[00:56:05]     Error {
[00:56:05]         line_num: 17,
[00:56:05]         kind: Some(
[00:56:05]             Error
[00:56:05]             Error
[00:56:05]         ),
[00:56:05]         msg: "17:1: 17:45: could not evaluate static initializer [E0080]"
[00:56:05] ]
[00:56:05] 
[00:56:05] not found errors (from test file): [
[00:56:05]     Error {
[00:56:05]     Error {
[00:56:05]         line_num: 17,
[00:56:05]         kind: Some(
[00:56:05]             Error
[00:56:05]         ),
[00:56:05]         msg: "constant evaluation error"
[00:56:05] ]
[00:56:05] 
[00:56:05] thread '[compile-fail] compile-fail/issue-28324.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:05] 
[00:56:05] 
[00:56:05] 
[00:56:05] failures:
[00:56:05]     [compile-fail] compile-fail/issue-14227.rs
[00:56:05]     [compotstrap/debug/bootstrap test
[00:56:05] Build completed unsuccessfully in 0:14:44
[00:56:05] make: *** [check] Error 1
[00:56:05] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f4c3884
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
