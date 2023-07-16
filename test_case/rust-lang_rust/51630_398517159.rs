plain
[00:55:48] ....................................................................................................
[00:55:54] ....................................................................................................
[00:56:02] ....................................................................................................
[00:56:07] ...i................................................................................................
[00:56:14] ..i..ii...................................................................................FFF.F.....
[00:56:27] ....................................................................................................
[00:56:31] ....................................................................................i...............
[00:56:36] ..............................i.....................................................................
[00:56:42] ....................................................................................................
[00:56:42] ....................................................................................................
[00:56:47] ....................................................................................................
[00:56:51] ....................................................................................................
[00:56:52] ..........................
[00:56:52] failures:
[00:56:52] 
[00:56:52] ---- [compile-fail] compile-fail/not-panic-safe-2.rs stdout ----
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-2.rs:20: unexpected error: '20:5: 20:31: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-2.rs:20: expected error not found: `std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied
[00:56:52] 
[00:56:52] error: 1 unexpected errors found, 1 expected errors not found
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/not-panic-safe-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-2/auxiliary" "-A" "unused"
[00:56:52] unexpected errors (from JSON output): [
[00:56:52]     Error {
[00:56:52]         line_num: 20,
[00:56:52]         kind: Some(
[00:56:52]         ),
[00:56:52]         ),
[00:56:52]         msg: "20:5: 20:31: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]"
[00:56:52] ]
[00:56:52] 
[00:56:52] not found errors (from test file): [
[00:56:52]     Error {
[00:56:52]     Error {
[00:56:52]         line_num: 20,
[00:56:52]         kind: Some(
[00:56:52]             Error
[00:56:52]         ),
[00:56:52]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[00:56:52] ]
[00:56:52] 
[00:56:52] thread '[compile-fail] compile-fail/not-panic-safe-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:52] 
[00:56:52] 
[00:56:52] ---- [compile-fail] compile-fail/not-panic-safe-3.rs stdout ----
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-3.rs:20: unexpected error: '20:5: 20:32: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-3.rs:20: expected error not found: `std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied
[00:56:52] 
[00:56:52] error: 1 unexpected errors found, 1 expected errors not found
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/not-panic-safe-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-3/auxiliary" "-A" "unused"
[00:56:52] unexpected errors (from JSON output): [
[00:56:52]     Error {
[00:56:52]         line_num: 20,
[00:56:52]         kind: Some(
[00:56:52]         ),
[00:56:52]         ),
[00:56:52]         msg: "20:5: 20:32: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]"
[00:56:52] ]
[00:56:52] 
[00:56:52] not found errors (from test file): [
[00:56:52]     Error {
[00:56:52]     Error {
[00:56:52]         line_num: 20,
[00:56:52]         kind: Some(
[00:56:52]             Error
[00:56:52]         ),
[00:56:52]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[00:56:52] ]
[00:56:52] 
[00:56:52] thread '[compile-fail] compile-fail/not-panic-safe-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:52] 
[00:56:52] 
[00:56:52] ---- [compile-fail] compile-fail/not-panic-safe-4.rs stdout ----
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-4.rs:19: unexpected error: '19:5: 19:28: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-4.rs:19: expected error not found: `std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied
[00:56:52] 
[00:56:52] error: 1 unexpected errors found, 1 expected errors not found
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/not-panic-safe-4.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-4/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-4/auxiliary" "-A" "unused"
[00:56:52] unexpected errors (from JSON output): [
[00:56:52]     Error {
[00:56:52]         line_num: 19,
[00:56:52]         kind: Some(
[00:56:52]         ),
[00:56:52]         ),
[00:56:52]         msg: "19:5: 19:28: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]"
[00:56:52] ]
[00:56:52] 
[00:56:52] not found errors (from test file): [
[00:56:52]     Error {
[00:56:52]     Error {
[00:56:52]         line_num: 19,
[00:56:52]         kind: Some(
[00:56:52]             Error
[00:56:52]         ),
[00:56:52]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[00:56:52] ]
[00:56:52] 
[00:56:52] thread '[compile-fail] compile-fail/not-panic-safe-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:52] 
[00:56:52] 
[00:56:52] ---- [compile-fail] compile-fail/not-panic-safe-6.rs stdout ----
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-6.rs:19: unexpected error: '19:5: 19:32: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[00:56:52] 
[00:56:52] error: /checkout/src/test/compile-fail/not-panic-safe-6.rs:19: expected error not found: `std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied
[00:56:52] 
[00:56:52] error: 1 unexpected errors found, 1 expected errors not found
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/not-panic-safe-6.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-6/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-6/auxiliary" "-A" "unused"
[00:56:52] unexpected errors (from JSON output): [
[00:56:52]     Error {
[00:56:52]         line_num: 19,
[00:56:52]         kind: Some(
[00:56:52]         ),
[00:56:52]         ),
[00:56:52]         msg: "19:5: 19:32: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]"
[00:56:52] ]
[00:56:52] 
[00:56:52] not found errors (from test file): [
[00:56:52]     Error {
[00:56:52]     Error {
[00:56:52]         line_num: 19,
[00:56:52]         kind: Some(
[00:56:52]             Error
[00:56:52]         ),
[00:56:52]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[00:56:52] ]
[00:56:52] 
[00:56:52] thread '[compile-fail] compile-fail/not-panic-safe-6.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:52] 
---
[00:56:52] 
[00:56:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:52] 
[00:56:52] 
[00:56:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:52] 
[00:56:52] 
[00:56:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:52] Build completed unsuccessfully in 0:14:50
[00:56:52] Build completed unsuccessfully in 0:14:50
[00:56:52] Makefile:58: recipe for target 'check' failed
[00:56:52] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ea6b18
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
