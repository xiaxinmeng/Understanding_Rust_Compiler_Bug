plain
[00:44:10] ...........................................................i........................................
[00:44:14] ....................................................................................................
[00:44:19] ....................................................................................................
[00:44:25] ......................................................................................i.............
[00:44:27] ....iiiiiiiii...................................................
[00:44:27] 
[00:44:27] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:45:13] ...........................................................i........................................
[00:45:17] ....................................................................................................
[00:45:22] ....................................................................................................
[00:45:27] ......................................................................................i.............
[00:45:30] ....iiiiiiiii...................................................
[00:45:30] 
[00:45:30]  finished in 62.380
[00:45:30] travis_fold:end:test_ui_nll

---
[00:55:30] 
[00:55:30] running 271 tests
[00:55:31] ...........................i........................................................................
[00:55:32] ....................................................................i...............................
[00:55:33] ...............i..........................................F............
[00:55:33] 
[00:55:33] ---- [parse-fail] parse-fail/unicode-chars.rs stdout ----
[00:55:33] 
[00:55:33] 
[00:55:33] error: /checkout/src/test/parse-fail/unicode-chars.rs:14: unexpected help message: '14:14: 14:15: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not'
[00:55:33] 
[00:55:33] error: /checkout/src/test/parse-fail/unicode-chars.rs:14: expected help message not found: unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
[00:55:33] 
[00:55:33] error: 1 unexpected errors found, 1 expected errors not found
[00:55:33] status: exit code: 101
[00:55:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/unicode-chars.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/unicode-chars/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/unicode-chars/auxiliary"
[00:55:33] unexpected errors (from JSON output): [
[00:55:33]     Error {
[00:55:33]         line_num: 14,
[00:55:33]         kind: Some(
[00:55:33]         ),
[00:55:33]         ),
[00:55:33]         msg: "14:14: 14:15: Unicode character \';\' (Greek Question Mark) looks like \';\' (Semicolon), but it is not"
[00:55:33] ]
[00:55:33] 
[00:55:33] not found errors (from test file): [
[00:55:33]     Error {
[00:55:33]     Error {
[00:55:33]         line_num: 14,
[00:55:33]         kind: Some(
[00:55:33]             Help
[00:55:33]         ),
[00:55:33]         msg: "unicode character \';\' (Greek Question Mark) looks like \';\' (Semicolon), but it is not"
[00:55:33] ]
[00:55:33] 
[00:55:33] thread '[parse-fail] parse-fail/unicode-chars.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:55:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:55:33] 
[00:55:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:33] 
[00:55:33] 
[00:55:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:33] 
[00:55:33] 
[00:55:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:33] Build completed unsuccessfully in 0:13:34
[00:55:33] Build completed unsuccessfully in 0:13:34
[00:55:33] Makefile:58: recipe for target 'check' failed
[00:55:33] make: *** [check] Error 1
104168 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
104164 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
103604 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
103236 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103236 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103232 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1e2or17jk-1pp4p6b-17uifkj4ft9sj
92244 ./obj/build/x86_64-unknown-linux-gnu/stage1
92220 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90724 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
89804 ./src/llvm/test/CodeGen
