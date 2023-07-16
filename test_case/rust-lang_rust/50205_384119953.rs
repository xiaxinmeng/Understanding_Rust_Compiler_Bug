plain
[00:57:13] 
[00:57:13] running 271 tests
[00:57:15] ...........................i........................................................................
[00:57:16] ....................................................................i...............................
[00:57:17] ...............i...............................F.......................
[00:57:17] 
[00:57:17] ---- [parse-fail] parse-fail/trait-object-lifetime-parens.rs stdout ----
[00:57:17]  
[00:57:17]  
[00:57:17] error: /checkout/src/test/parse-fail/trait-object-lifetime-parens.rs:13: unexpected error: '13:19: 13:20: expected one of `+`, `,`, `=`, or `>`, found `)`'
[00:57:17] 
[00:57:17] error: /checkout/src/test/parse-fail/trait-object-lifetime-parens.rs:16: expected error not found: parenthesized lifetime bounds are not supported
[00:57:17] 
[00:57:17] error: /checkout/src/test/parse-fail/trait-object-lifetime-parens.rs:17: expected error not found: expected type, found `'a`
[00:57:17] 
[00:57:17] error: 1 unexpected errors found, 2 expected errors not found
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/trait-object-lifetime-parens.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-object-lifetime-parens.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-object-lifetime-parens.stage2-x86_64-unknown-linux-gnu.aux"
[00:57:17] unexpected errors (from JSON output): [
[00:57:17]     Error {
[00:57:17]         line_num: 13,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "13:19: 13:20: expected one of `+`, `,`, `=`, or `>`, found `)`"
[00:57:17] ]
[00:57:17] 
[00:57:17] not found errors (from test file): [
[00:57:17]     Error {
[00:57:17]     Error {
[00:57:17]         line_num: 16,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "parenthesized lifetime bounds are not supported"
[00:57:17]     Error {
[00:57:17]         line_num: 17,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "expected type, found `\'a`"
[00:57:17] ]
[00:57:17] 
[00:57:17] thread '[parse-fail] parse-fail/trait-object-lifetime-parens.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1274:13
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:57:17] 
[00:57:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:57:17] 
[00:57:17] 
[00:57:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:17] 
[00:57:17] 
[00:57:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:17] Build completed unsuccessfully in 0:17:08
[00:57:17] Build completed unsuccessfully in 0:17:08
[00:57:17] make: *** [check] Error 1
[00:57:17] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0460ea56
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
