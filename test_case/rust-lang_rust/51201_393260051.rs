plain
[01:03:31] ......................................................................i.............................
[01:03:36] ....................................................................................................
[01:03:44] ....................................................................................................
[01:03:52] ...................................................................................................i
[01:03:56] .................iiiiiiiii...................................................
[01:03:56] 
[01:03:56] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[01:04:58] ......................................................................i.............................
[01:05:04] ....................................................................................................
[01:05:10] ....................................................................................................
[01:05:18] ...................................................................................................i
[01:05:22] .................iiiiiiiii...................................................
[01:05:22] 
[01:05:22]  finished in 85.888
[01:05:22] travis_fold:end:test_ui_nll

---
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:59] 
[01:20:59] running 271 tests
[01:21:01] ..................F........i........................................................................
[01:21:03] ...............i.......................................................
[01:21:03] failures:
[01:21:03] 
[01:21:03] ---- [parse-fail] parse-fail/bind-struct-early-modifiers.rs stdout ----
[01:21:03] ---- [parse-fail] parse-fail/bind-struct-early-modifiers.rs stdout ----
[01:21:03] 
[01:21:03] error: /checkout/src/test/parse-fail/bind-struct-early-modifiers.rs:16: unexpected error: '16:19: 16:20: expected `,`'
[01:21:03] 
[01:21:03] error: /checkout/src/test/parse-fail/bind-struct-early-modifiers.rs:16: expected error not found: expected `,`, found `:`
[01:21:03] 
[01:21:03] error: 1 unexpected errors found, 1 expected errors not found
[01:21:03] status: exit code: 101
[01:21:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/bind-struct-early-modifiers.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/bind-struct-early-modifiers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/bind-struct-early-modifiers/auxiliary"
[01:21:03] unexpected errors (from JSON output): [
[01:21:03]     Error {
[01:21:03]         line_num: 16,
[01:21:03]         kind: Some(
[01:21:03]         ),
[01:21:03]         ),
[01:21:03]         msg: "16:19: 16:20: expected `,`"
[01:21:03] ]
[01:21:03] 
[01:21:03] not found errors (from test file): [
[01:21:03]     Error {
[01:21:03]     Error {
[01:21:03]         line_num: 16,
[01:21:03]         kind: Some(
[01:21:03]             Error
[01:21:03]         ),
[01:21:03]         msg: "expected `,`, found `:`"
[01:21:03] ]
[01:21:03] 
[01:21:03] thread '[parse-fail] parse-fail/bind-struct-early-modifiers.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:21:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:21:03] test result: FAILED. 267 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[01:21:03] 
[01:21:03] 
[01:21:03] 
[01:21:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:03] 
[01:21:03] 
[01:21:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:03] Build completed unsuccessfully in 0:20:36
[01:21:03] Build completed unsuccessfully in 0:20:36
[01:21:03] Makefile:58: recipe for target 'check' failed
[01:21:03] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08b02870
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
