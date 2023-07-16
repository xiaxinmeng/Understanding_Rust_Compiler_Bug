plain
[01:00:26] ....................................................................................................
[01:00:32] ....................................................................................................
[01:00:40] ....................................................................................................
[01:00:45] ...i................................................................................................
[01:00:52] ..i..ii...................................................................................FFF.F.....
[01:01:06] ....................................................................................................
[01:01:09] .........................................................test [compile-fail] compile-fail/issue-22638.rs has been running for over 60 seconds
[01:01:12] ...........................i...............
[01:01:16] ..............................i.....................................................................
---
[01:01:32] failures:
[01:01:32] 
[01:01:32] ---- [compile-fail] compile-fail/not-panic-safe-2.rs stdout ----
[01:01:32] 
[01:01:32] error: /checkout/src/test/compile-fail/not-panic-safe-2.rs:20: unexpected error: '20:5: 20:31: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[01:01:32] 
[01:01:32] error: /checkout/src/test/compile-fail/not-panic-safe-2.rs:20: expected error not found: `std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied
[01:01:32] 
[01:01:32] error: 1 unexpected errors found, 1 expected errors not found
[01:01:32] status: exit code: 101
[01:01:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/not-panic-safe-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-2/auxiliary" "-A" "unused"
[01:01:32] unexpected errors (from JSON output): [
[01:01:32]     Error {
[01:01:32]         line_num: 20,
[01:01:32]         kind: Some(
[01:01:32]         ),
[01:01:32]         ),
[01:01:32]         msg: "20:5: 20:31: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]"
[01:01:32] ]
[01:01:32] 
[01:01:32] not found errors (from test file): [
[01:01:32]     Error {
[01:01:32]     Error {
[01:01:32]         line_num: 20,
[01:01:32]         kind: Some(
[01:01:32]             Error
[01:01:32]         ),
[01:01:32]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[01:01:32] ]
[01:01:32] 
[01:01:32] thread '[compile-fail] compile-fail/not-panic-safe-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:01:32] 
[01:01:32] 
[01:01:32] ---- [compile-fail] compile-fail/not-panic-safe-3.rs stdout ----
[01:01:32] 
[01:01:32] error: /checkout/src/test/compile-fail/not-panic-safe-3.rs:20: unexpected error: '20:5: 20:32: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[01:01:32] 
[01:01:32] error: /checkout/src/test/compile-fail/not-panic-safe-3.rs:20: expected error not found: `std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied
[01:01:32] 
[01:01:32] error: 1 unexpected errors found, 1 expected errors not found
[01:01:32] status: exit code: 101
[01:01:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/not-panic-safe-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/not-panic-safe-3/auxiliary" "-A" "unused"
[01:01:32] unexpected errors (from JSON output): [
[01:01:32]     Error {
[01:01:32]         line_num: 20,
[01:01:32]         kind: Some(
[01:01:32]         ),
[01:01:32]         ),
[01:01:32]         msg: "20:5: 20:32: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]"
[01:01:32] ]
[01:01:32] 
[01:01:32] not found errors (from test file): [
[01:01:32]     Error {
[01:01:32]     Error {
[01:01:32]         line_num: 20,
[01:01:32]         kind: Some(
[01:01:32]             Error
[01:01:32]         ),
[01:01:32]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[01:01:32] ]
[01:01:32] 
[01:01:32] thread '[compile-fail] compile-fail/not-panic-safe-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:01:32] 
[01:01:32] 
[01:01:32] ---- [compile-fail] compile-fail/not-panic-safe-4.rs stdout ----
[01:01:32] 
[01:01:32] error: /checkout/src/test/compile-fail/not-panic-safe-4.rs:19: unexpected error: '19:5: 19:28: the trait bound `std::cell::UnsafeCell<isize>: std::panic::RefUnwindSafe` is not satisfied in `std::cell::RefCell<i32>` [E0277]'
[01:01:32] 
[01:01:32] error: /checkout/src/test/compile-fail/not-panic-safe-4.rs:19: expectt file): [
[01:01:32]     Error {
[01:01:32]         line_num: 19,
[01:01:32]         kind: Some(
[01:01:32]         ),
[01:01:32]         ),
[01:01:32]         msg: "`std::cell::UnsafeCell<usize>: std::panic::RefUnwindSafe` is not satisfied"
[01:01:32] ]
[01:01:32] 
[01:01:32] thread '[compile-fail] compile-fail/not-panic-safe-6.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[01:01:32] 
---
[01:01:32] 
[01:01:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:01:32] 
[01:01:32] 
[01:01:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:32] 
[01:01:32] 
[01:01:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:32] Build completed unsuccessfully in 0:14:55
[01:01:32] Build completed unsuccessfully in 0:14:55
[01:01:32] Makefile:58: recipe for target 'check' failed
[01:01:32] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:092e51aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
