plain
travis_time:start:test_compile-fail
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:12] 
[01:03:12] running 2311 tests
[01:03:17] ..............F.F...................................................................................
[01:03:30] ....................................................................................................
[01:03:37] ....................................................................................................
[01:03:37] ....................................................................................................
[01:03:44] ........F...........i............................................................ii.iii.............
[01:03:50] .........................................................F..........................................
[01:04:04] ....................................................................................................
[01:04:10] ............i.......................................................................................
[01:04:17] ....................................................................................................
[01:04:25] ....................................................................................................
---
[01:06:17] failures:
[01:06:17] 
[01:06:17] ---- [compile-fail] compile-fail/array_const_index-0.rs stdout ----
[01:06:17]  
[01:06:17] error: /checkout/src/test/compile-fail/array_const_index-0.rs:12: unexpected error: '12:1: 12:24: this constant cannot be used [const_err]'
[01:06:17] 
[01:06:17] error: /checkout/src/test/compile-fail/array_const_index-0.rs:12: expected error not found: constant evaluation error
[01:06:17] 
[01:06:17] error: /checkout/src/test/compile-fail/array_const_index-0.rs:12: expected warning not found: this constant cannot be used
[01:06:17] 
[01:06:17] error: 1 unexpected errors found, 2 expected errors not found
[01:06:17] status: exit code: 101
[01:06:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/array_const_index-0.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/array_const_index-0.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/array_const_index-0.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:06:17] unexpected errors (from JSON output): [
[01:06:17]     Error {
[01:06:17]         line_num: 12,
[01:06:17]         kind: Some(
[01:06:17]         ),
[01:06:17]         ),
[01:06:17]         msg: "12:1: 12:24: this constant cannot be used [const_err]"
[01:06:17] ]
[01:06:17] 
[01:06:17] not found errors (from test file): [
[01:06:17]     Error {
[01:06:17]     Error {
[01:06:17]         line_num: 12,
[01:06:17]         kind: Some(
[01:06:17]             Error
[01:06:17]         ),
[01:06:17]         msg: "constant evaluation error"
[01:06:17]     Error {
[01:06:17]         line_num: 12,
[01:06:17]         kind: Some(
[01:06:17]             Warning
[01:06:17]             Warning
[01:06:17]         ),
[01:06:17]         msg: "this constant cannot be used"
[01:06:17] ]
[01:06:17] 
[01:06:17] thread '[compile-fail] compile-fail/array_const_index-0.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:06:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:17] 
[01:06:17] ---- [compile-fail] compile-fail/array_const_index-1.rs stdout ----
[01:06:17]  
[01:06:17] error: /checkout/src/test/compile-fail/array_const_index-1.rs:12: unexpected error: '12:1: 12:21: this constant cannot be used [const_err]'
[01:06:17] 
[01:06:17] error: /checkout/src/test/compile-fail/array_const_index-1.rs:12: expected error not found: constant evaluation error
[01:06:17] 
[01:06:17] error: /checkout/src/test/compile-fail/array_const_index-1.rs:12: expected warning not found: this constant cannot be used
[01:06:17] 
[01:06:17] error: 1 unexpected errors found, 2 expected errors not found
[01:06:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stagt-slice-oob.rs stdout ----
[01:06:17]  
[01:06:17]  
[01:06:17] error: /checkout/src/test/compile-fail/const-slice-oob.rs:14: unexpected error: '14:1: 14:25: this constant cannot be used [const_err]'
[01:06:17] 
[01:06:17] error: /checkout/src/test/compile-fail/const-slice-oob.rs:14: expected error not found: constant evaluation error [E0080]
[01:06:17] 
[01:06:17] error: /checkout/src/test/compile-fail/const-slice-oob.rs:14: expected warning not found: this constant cannot be used
[01:06:17] 
[01:06:17] error: 1 unexpected errors found, 2 expected errors not found
[01:06:17] status: exit code: 101
[01:06:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-slice-oob.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-slice-oob.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-slice-oob.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:06:17] unexpected errors (from JSON output): [
[01:06:17]     Error {
[01:06:17]         line_num: 14,
[01:06:17]         kind: Some(
[01:06:17]         ),
[01:06:17]         ),
[01:06:17]         msg: "14:1: 14:25: this constant cannot be used [const_err]"
[01:06:17] ]
[01:06:17] 
[01:06:17] not found errors (from test file): [
[01:06:17] not found errors (from test file): [
[01: "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/eval-enum.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/eval-enum.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:06:17] unexpected errors (from JSON output): [
[01:06:17]     Error {
[01:06:17]         line_num: 12,
[01:06:17]         kind: Some(
[01:06:17]         ),
[01:06:17]         ),
[01:06:17]         msg: "12:15: 12:18: constant evaluation error [E0080]"
[01:06:17]     Error {
[01:06:17]         line_num: 16,
[01:06:17]         kind: Some(
[01:06:17]             Error
[01:06:17]             Error
[01:06:17]         ),
[01:06:17]         msg: "16:15: 16:18: constant evaluation error [E0080]"
[01:06:17] ]
[01:06:17] 
[01:06:17] not found errors (from test file): [
[01:06:17]     Error {
[01:06:17]     Error {
[01:06:17]         line_num: 12,
[01:06:17]         kind: Some(
[01:06:17]             Warning
[01:06:17]         ),
[01:06:17]         msg: "constant evaluation error"
[01:06:17]     Error {
[01:06:17]         line_num: 16,
[01:06:17]         kind: Some(
[01:06:17]             Warning
[01:06:17]             Warning
[01:06:17]         ),
[01:06:17]         msg: "constant evaluation error"
[01:06:17] ]
[01:06:17] 
[01:06:17] thread '[compile-fail] compile-fail/eval-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:06:17] 
---
[01:06:17] 
[01:06:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:06:17] 
[01:06:17] 
[01:06:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:17] 
[01:06:17] 
[01:06:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:17] Build completed unsuccessfully in 0:17:27
[01:06:17] Build completed unsuccessfully in 0:17:27
[01:06:17] Makefile:58: recipe for target 'check' failed
[01:06:17] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11a152f7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149800 ./.git/modules
149796 ./.git/modules/src
149116 ./src/llvm-emscripten/test
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0xq7o24gx-1vn2dho-22v3gsapbxtg7
116624 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x8./src/libcompiler_builtins
39456 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
39368 ./src/llvm/lib/Target
38828 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/build
35996 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps
