plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:18] 
[00:46:18] running 1376 tests
[00:46:22] ..................................................................................i.................
[00:46:28] .........F.F...F....F.F...F...i.....................................................................
[00:46:35] ....................................................................................................
[00:46:39] ....................................................................................................
[00:46:43] ....................................................................................................
[00:46:48] ....................................................................................................
---
[00:47:27] failures:
[00:47:27] 
[00:47:27] ---- [ui] ui/const-eval-overflow-4.rs stdout ----
[00:47:27]  
[00:47:27] error: /checkout/src/test/ui/const-eval-overflow-4.rs:23: expected warning not found: attempt to add with overflow
[00:47:27] 
[00:47:27] error: 0 unexpected errors found, 1 expected errors not found
[00:47:27] status: exit code: 101
[00:47:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval-overflow-4.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval-overflow-4.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval-overflow-4.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:27] not found errors (from test file): [
[00:47:27]     Error {
[00:47:27]         line_num: 23,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]         ),
[00:47:27]         msg: "attempt to add with overflow"
[00:47:27] ]
[00:47:27] 
[00:47:27] thread '[ui] ui/const-eval-overflow-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[00:47:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:27] 
[00:47:27] ---- [ui] ui/const-eval/conditional_array_execution.rs stdout ----
[00:47:27]  
[00:47:27] error: /checkout/src/test/ui/const-eval/conditional_array_execution.rs:15: expected warning not found: attempt to subtract with overflow
[00:47:27] 
[00:47:27] error: 0 unexpected errors found, 1 expected errors not found
[00:47:27] status: exit code: 0
[00:47:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/conditional_array_execution.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/conditional_array_execution.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/conditional_array_execution.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:27] not found errors (from test file): [
[00:47:27]     Error {
[00:47:27]         line_num: 15,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]         ),
[00:47:27]         msg: "attempt to subtract with overflow"
[00:47:27] ]
[00:47:27] 
[00:47:27] thread '[ui] ui/const-eval/conditional_array_execution.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[00:47:27] 
[00:47:27] 
[00:47:27] ---- [ui] ui/const-eval/issue-43197.rs stdout ----
[00:47:27]  
[00:47:27] error: /checkout/src/test/ui/const-eval/issue-43197.rs:20: expected warning not found: attempt to subtract with overflow
[00:47:27] 
[00:47:27] error: /checkout/src/test/ui/const-eval/issue-43197.rs:23: expected warning not found: attempt to subtract with overflow
[00:47:27] 
[00:47:27] error: 0 unexpected errors found, 2 expected errors not found
[00:47:27] status: exit code: 0
[00:47:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/issue-43197.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-43197.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/issue-43197.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:27] not found errors (from test file): [
[00:47:27]     Error {
[00:47:27]         line_num: 20,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]         ),
[00:47:27]         msg: "attempt to subtract with overflow"
[00:47:27]     Error {
[00:47:27]         line_num: 23,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]             Warning
[00:47:27]         ),
[00:47:27]         msg: "attempt to subtract with overflow"
[00:47:27] ]
[00:47:27] 
[00:47:27] thread '[ui] ui/const-eval/issue-43197.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[00:47:27] 
[00:47:27] 
[00:47:27] ---- [ui] ui/const-eval/pub_const_err.rs stdout ----
[00:47:27]  
[00:47:27] error: /checkout/src/test/ui/const-eval/pub_const_err.rs:15: expected warning not found: attempt to subtract with overflow
[00:47:27] 
[00:47:27] error: /checkout/src/test/ui/const-eval/pub_const_err.rs:19: expected warning not found: attempt to subtract with overflow
[00:47:27] 
[00:47:27] error: 0 unexpected errors found, 2 expected errors not found
[00:47:27] status: exit code: 0
[00:47:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/pub_const_err.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/pub_const_errget=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/pub_const_err_bin.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/pub_const_err_bin.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:27] not found errors (from test file): [
[00:47:27]     Error {
[00:47:27]         line_num: 13,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]         ),
[00:47:27]         msg: "attempt to subtract with overflow"
[00:47:27]     Error {
[00:47:27]         line_num: 17,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]             Warning
[00:47:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:47:27]         ),
[00:47:27]         msg: "attempt to subtract with overflow"
[00:47:27] ]
[00:47:27] 
[00:47:27] thread '[ui] ui/const-eval/pub_const_err_bin.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[00:47:27] 
[00:47:27] 
[00:47:27] ---- [ui] ui/const-len-underflow-separate-spans.rs stdout ----
[00:47:27]  
[00:47:27] error: /checkout/src/test/ui/const-len-underflow-separate-spans.rs:17: expected warning not found: attempt to subtract with overflow
[00:47:27] 
[00:47:27] error: 0 unexpected errors found, 1 expected errors not found
[00:47:27] status: exit code: 101
[00:47:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-len-underflow-separate-spans.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-len-underflow-separate-spans.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-len-underflow-separate-spans.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:27] not found errors (from test file): [
[00:47:27]     Error {
[00:47:27]         line_num: 17,
[00:47:27]         kind: Some(
[00:47:27]             Warning
[00:47:27]         ),
[00:47:27]         msg: "attempt to subtract with overflow"
[00:47:27] ]
[00:47:27] 
[00:47:27] thread '[ui] ui/const-len-underflow-separate-spans.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[00:47:27] 
---
[00:47:27] test result: FAILED. 1363 passed; 6 failed; 7 ignored; 0 measured; 0 filtered out
[00:47:27] 
[00:47:27] 
[00:47:27] 
[00:47:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:27] 
[00:47:27] 
[00:47:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:27] Build completed unsuccessfully in 0:02:18
[00:47:27] Build completed unsuccessfully in 0:02:18
[00:47:27] Makefile:58: recipe for target 'check' failed
[00:47:27] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002b1b68
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
