plain
[01:00:58] ......................................................................................i.............
[01:01:04] ...............................................ii..iii..............................................
[01:01:09] ..............................................................................i.....................
[01:01:15] .......................i............................................................................
[01:01:19] ...........................................................F..................................i.....
[01:01:30] ....................................................................................................
[01:01:36] F...................................................................................................
[01:01:42] ....................................................................................................
[01:01:49] ....................................................................................................
---
[01:02:50] failures:
[01:02:50] 
[01:02:50] ---- [compile-fail] compile-fail/issue-14227.rs stdout ----
[01:02:50] 
[01:02:50] error: /checkout/src/test/compile-fail/issue-14227.rs:16: unexpected error: '16:20: 16:26: could not evaluate static initializer [E0080]'
[01:02:50] 
[01:02:50] error: /checkout/src/test/compile-fail/issue-14227.rs:16: unexpected error: '16:1: 16:27: could not evaluate static initializer [E0080]'
[01:02:50] 
[01:02:50] error: /checkout/src/test/compile-fail/issue-14227.rs:16: expected error not found: constant evaluation error
[01:02:50] 
[01:02:50] error: 2 unexpected errors found, 1 expected errors not found
[01:02:50] status: exit code: 101
[01:02:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-14227.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/auxiliary" "-A" "unused"
[01:02:50] unexpected errors (from JSON output): [
[01:02:50]     Error {
[01:02:50]         line_num: 16,
[01:02:50]         kind: Some(
[01:02:50]         ),
[01:02:50]         ),
[01:02:50]         msg: "16:20: 16:26: could not evaluate static initializer [E0080]"
[01:02:50]     Error {
[01:02:50]         line_num: 16,
[01:02:50]         kind: Some(
[01:02:50]             Error
[01:02:50]             Error
[01:02:50]         ),
[01:02:50]         msg: "16:1: 16:27: could not evaluate static initializer [E0080]"
[01:02:50] ]
[01:02:50] 
[01:02:50] not found errors (from test file): [
[01:02:50]     Error {
[01:02:50]     Error {
[01:02:50]         line_num: 16,
[01:02:50]         kind: Some(
[01:02:50]             Error
[01:02:50]         ),
[01:02:50]         msg: "constant evaluation error"
[01:02:50] ]
[01:02:50] 
[01:02:50] thread '[compile-fail] compile-fail/issue-14227.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:02:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:50] 
[01:02:50] ---- [compile-fail] compile-fail/issue-28324.rs stdout ----
[01:02:50] 
[01:02:50] error: /checkout/src/test/compile-fail/issue-28324.rs:17: unexpected error: '17:23: 17:44: could not evaluate static initializer [E0080]'
[01:02:50] 
[01:02:50] error: /checkout/src/test/compile-fail/issue-28324.rs:17: unexpected error: '17:1: 17:45: could not evaluate static initializer [E0080]'
[01:02:50] 
[01:02:50] error: /checkout/src/test/compile-fail/issue-28324.rs:17: expected error not found: constant evaluation error
[01:02:50] 
[01:02:50] error: 2 unexpected errors found, 1 expected errors not found
[01:02:50] status: exit code: 101
[01:02:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-28324.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/auxiliary" "-A" "unused"
[01:02:50] unexpected errors (from JSON output): [
[01:02:50]     Error {
[01:02:50]         line_num: 17,
[01:02:50]         kind: Some(
[01:02:50]         ),
[01:02:50]         ),
[01:02:50]         msg: "17:23: 17:44: could not evaluate static initializer [E0080]"
[01:02:50]     Error {
[01:02:50]         line_num: 17,
[01:02:50]         kind: Some(
[01:02:50]             Error
[01:02:50]             Error
[01:02:50]         ),
[01:02:50]         msg: "17:1: 17:45: could not evaluate static initializer [E0080]"
[01:02:50] ]
[01:02:50] 
[01:02:50] not found errors (from test file): [
[01:02:50]     Error {
[01:02:50]     Error {
[01:02:50]         line_num: 17,
[01:02:50]         kind: Some(
[01:02:50]             Error
[01:02:50]         ),
[01:02:50]         msg: "constant evaluation error"
[01:02:50] ]
[01:02:50] 
[01:02:50] thread '[compile-fail] compile-fail/issue-28324.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:02:50] 
---
[01:02:50] 
[01:02:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:02:50] 
[01:02:50] 
[01:02:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:50] 
[01:02:50] 
[01:02:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:50] Build completed unsuccessfully in 0:15:30
[01:02:50] Build completed unsuccessfully in 0:15:30
[01:02:50] Makefile:58: recipe for target 'check' failed
[01:02:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:033d7a78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
