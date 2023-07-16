plain
[00:54:11] ...................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:35] .................................................................................
[00:54:55] ......................................................................................ii............
[00:55:46] ..................................................i.................................................
[00:55:57] ...i.ii..........................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:56:41] ...........iiiiiii..................................................................................
[00:57:02] ....................................................................................................
[00:57:21] ....................................................................................................
[00:57:40] ....................................................................................
---
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:45] 
[01:00:45] running 271 tests
[01:00:46] ...........................i....................................................F.F.................
[01:00:48] ....................................................................i...............................
[01:00:49] ...............i................F......................................
[01:00:49] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:00:49] 
[01:00:49] ---- [parse-fail] parse-fail/issue-20711-2.rs stdout ----
[01:00:49]  
[01:00:49]  
[01:00:49] error: /checkout/src/test/parse-fail/issue-20711-2.rs:19: unexpected error: '19:1: 19:2: expected one of `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`'
[01:00:49] 
[01:00:49] error: /checkout/src/test/parse-fail/issue-20711-2.rs:19: expected error not found: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`
[01:00:49] 
[01:00:49] error: 1 unexpected errors found, 1 expected errors not found
[01:00:49] status: exit code: 101
[01:00:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/issue-20711-2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711-2.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711-2.stage2-x86_64-unknown-linux-gnu.aux"
[01:00:49] unexpected errors (from JSON output): [
[01:00:49]     Error {
[01:00:49]         line_num: 19,
[01:00:49]         kind: Some(
[01:00:49]         ),
[01:00:49]         ),
[01:00:49]         msg: "19:1: 19:2: expected one of `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`"
[01:00:49] ]
[01:00:49] 
[01:00:49] not found errors (from test file): [
[01:00:49]     Error {
[01:00:49]     Error {
[01:00:49]         line_num: 19,
[01:00:49]         kind: Some(
[01:00:49]             Error
[01:00:49]         ),
[01:00:49]         msg: "expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`"
[01:00:49] ]
[01:00:49] 
[01:00:49] thread '[parse-fail] parse-fail/issue-20711-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:00:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:49] 
[01:00:49] ---- [parse-fail] parse-fail/issue-20711.rs stdout ----
[01:00:49]  
[01:00:49] error: /checkout/src/test/parse-fail/issue-20711.rs:17: unexpected error: '17:1: 17:2: expected one of `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`'
[01:00:49] 
[01:00:49] error: /checkout/src/test/parse-fail/issue-20711.rs:17: expected error not found: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`
[01:00:49] 
[01:00:49] error: 1 unexpected errors found, 1 expected errors not found
[01:00:49] status: exit code: 101
[01:00:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/issue-20711.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711.stage2-x86_64-unknown-linux-gnu.aux"
[01:00:49] unexpected errors (from JSON output): [
[01:00:49]     Error {
[01:00:49]         line_num: 17,
[01:00:49]         kind: Some(
[01:00:49]         ),
[01:00:49]         ),
[01:00:49]         msg: "17:1: 17:2: expected one of `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`"
[01:00:49] ]
[01:00:49] 
[01:00:49] not found errors (from test file): [
[01:00:49]     Error {
[01:00:49]     Error {
[01:00:49]         line_num: 17,
[01:00:49]         kind: Some(
[01:00:49]             Error
[01:00:49]         ),
[01:00:49]         msg: "expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`"
[01:00:49] ]
[01:00:49] 
[01:00:49] thread '[parse-fail] parse-fail/issue-20711.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:00:49] 
[01:00:49] 
[01:00:49] ---- [parse-fail] parse-fail/removed-syntax-static-fn.rs stdout ----
[01:00:49]  
[01:00:49] error: /checkout/src/test/parse-fail/removed-syntax-static-fn.rs:16: unexpected error: '16:5: 16:11: expected one of `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `static`'
[01:00:49] 
[01:00:49] error: /checkout/src/test/parse-fail/removed-syntax-static-fn.rs:16: expected error not found: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`
[01:00:49] 
[01:00:49] error: 1 unexpected errors found, 1 expected errors not found
[01:00:49] status: exit code: 101
[01:00:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/removed-syntax-static-fn.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/removed-syntax-static-fn.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/removed-syntax-static-fn.stage2-x86_64-unknown-linux-gnu.aux"
[01:00:49] unexpected errors (from JSON output): [
[01:00:49]     Error {
[01:00:49]         line_num: 16,
[01:00:49]         kind: Some(
[01:00:49]         ),
[01:00:49]         ),
[01:00:49]         msg: "16:5: 16:11: expected one of `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `static`"
[01:00:49] ]
[01:00:49] 
[01:00:49] not found errors (from test file): [
[01:00:49]     Error {
[01:00:49]     Error {
[01:00:49]         line_num: 16,
[01:00:49]         kind: Some(
[01:00:49]             Error
[01:00:49]         ),
[01:00:49]         msg: "expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`"
[01:00:49] ]
[01:00:49] 
[01:00:49] thread '[parse-fail] parse-fail/removed-syntax-static-fn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:00:49] 
---
[01:00:49] test result: FAILED. 265 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out
[01:00:49] 
[01:00:49] 
[01:00:49] 
[01:00:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:49] 
[01:00:49] 
[01:00:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:49] Build completed unsuccessfully in 0:16:57
[01:00:49] Build completed unsuccessfully in 0:16:57
[01:00:49] make: *** [check] Error 1
[01:00:49] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:131ef1dc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
