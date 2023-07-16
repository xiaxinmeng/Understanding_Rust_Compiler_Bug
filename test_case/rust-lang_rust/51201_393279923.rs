plain
[00:43:01] ......................................................................i.............................
[00:43:05] ....................................................................................................
[00:43:11] ....................................................................................................
[00:43:17] ...................................................................................................i
[00:43:20] .................iiiiiiiii...................................................
[00:43:20] 
[00:43:20] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:07] ......................................................................i.............................
[00:44:11] ....................................................................................................
[00:44:16] ....................................................................................................
[00:44:21] ...................................................................................................i
[00:44:25] .................iiiiiiiii...................................................
[00:44:25] 
[00:44:25]  finished in 64.769
[00:44:25] travis_fold:end:test_ui_nll

---
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:25] 
[00:54:25] running 271 tests
[00:54:26] ..................F........i........................................................................
[00:54:29] ...............i.......................................................
[00:54:29] failures:
[00:54:29] 
[00:54:29] ---- [parse-fail] parse-fail/bind-struct-early-modifiers.rs stdout ----
[00:54:29] ---- [parse-fail] parse-fail/bind-struct-early-modifiers.rs stdout ----
[00:54:29] 
[00:54:29] error: /checkout/src/test/parse-fail/bind-struct-early-modifiers.rs:16: unexpected error: '16:19: 16:20: expected `,`'
[00:54:29] 
[00:54:29] error: /checkout/src/test/parse-fail/bind-struct-early-modifiers.rs:16: expected error not found: expected `,`, found `:`
[00:54:29] 
[00:54:29] error: 1 unexpected errors found, 1 expected errors not found
[00:54:29] status: exit code: 101
[00:54:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/bind-struct-early-modifiers.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/bind-struct-early-modifiers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/bind-struct-early-modifiers/auxiliary"
[00:54:29] unexpected errors (from JSON output): [
[00:54:29]     Error {
[00:54:29]         line_num: 16,
[00:54:29]         kind: Some(
[00:54:29]         ),
[00:54:29]         ),
[00:54:29]         msg: "16:19: 16:20: expected `,`"
[00:54:29] ]
[00:54:29] 
[00:54:29] not found errors (from test file): [
[00:54:29]     Error {
[00:54:29]     Error {
[00:54:29]         line_num: 16,
[00:54:29]         kind: Some(
[00:54:29]             Error
[00:54:29]         ),
[00:54:29]         msg: "expected `,`, found `:`"
[00:54:29] ]
[00:54:29] 
[00:54:29] thread '[parse-fail] parse-fail/bind-struct-early-modifiers.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:54:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:54:29] test result: FAILED. 267 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:29] 
[00:54:29] 
[00:54:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:29] Build completed unsuccessfully in 0:13:38
[00:54:29] Build completed unsuccessfully in 0:13:38
[00:54:29] Makefile:58: recipe for target 'check' failed
[00:54:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12746c74
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
