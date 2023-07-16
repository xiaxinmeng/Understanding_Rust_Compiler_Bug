plain
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:34] 
[00:56:34] running 271 tests
[00:56:36] ...........................i.....................................................FF.................
[00:56:37] ....................................................................i...............................
[00:56:38] ...............i...............F.................F....F.FF.............
[00:56:38] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/issue-20711-2.rs stdout ----
[00:56:38] 
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/issue-20711-2.rs:19: unexpected error: '19:1: 19:2: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/issue-20711-2.rs:19: expected error not found: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/issue-20711-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711-2/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 19,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "19:1: 19:2: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 19,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/issue-20711-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/issue-20711.rs stdout ----
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/issue-20711.rs:17: unexpected error: '17:1: 17:2: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/issue-20711.rs:17: expected error not found: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/issue-20711.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-20711/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 17,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "17:1: 17:2: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `}`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 17,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/issue-20711.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] 
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/removed-syntax-static-fn.rs stdout ----
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/removed-syntax-static-fn.rs:16: unexpected error: '16:5: 16:11: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `static`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/removed-syntax-static-fn.rs:16: expected error not found: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/removed-syntax-static-fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/removed-syntax-static-fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/removed-syntax-static-fn/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 16,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "16:5: 16:11: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `static`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 16,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/removed-syntax-static-fn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] 
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/trait-non-item-macros.rs stdout ----
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-non-item-macros.rs:12: unexpected error: '12:19: 12:21: expected one of `async`, `const`, `extern`, `fn`, `type`, or `unsafe`, found `2`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-non-item-macros.rs:12: expected error not found: expected one of `const`, `extern`, `fn`, `type`, or `unsafe`, found `2`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/trait-non-item-macros.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-non-item-macros/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-non-item-macros/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "12:19: 12:21: expected one of `async`, `const`, `extern`, `fn`, `type`, or `unsafe`, found `2`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `extern`, `fn`, `type`, or `unsafe`, found `2`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/trait-non-item-macros.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] 
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/trait-pub-assoc-const.rs stdout ----
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-pub-assoc-const.rs:12: unexpected error: '12:5: 12:8: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-pub-assoc-const.rs:12: expected error not found: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/trait-pub-assoc-const.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-pub-assoc-const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-pub-assoc-const/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "12:5: 12:8: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/trait-pub-assoc-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] 
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/trait-pub-assoc-ty.rs stdout ----
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-pub-assoc-ty.rs:12: unexpected error: '12:5: 12:8: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-pub-assoc-ty.rs:12: expected error not found: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/trait-pub-assoc-ty.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-pub-assoc-ty/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-pub-assoc-ty/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "12:5: 12:8: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/trait-pub-assoc-ty.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] 
[00:56:38] 
[00:56:38] ---- [parse-fail] parse-fail/trait-pub-method.rs stdout ----
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-pub-method.rs:12: unexpected error: '12:5: 12:8: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`'
[00:56:38] 
[00:56:38] error: /checkout/src/test/parse-fail/trait-pub-method.rs:12: expected error not found: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`
[00:56:38] 
[00:56:38] error: 1 unexpected errors found, 1 expected errors not found
[00:56:38] status: exit code: 101
[00:56:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/trait-pub-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-pub-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-pub-method/auxiliary"
[00:56:38] unexpected errors (from JSON output): [
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]         ),
[00:56:38]         ),
[00:56:38]         msg: "12:5: 12:8: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`"
[00:56:38] ]
[00:56:38] 
[00:56:38] not found errors (from test file): [
[00:56:38]     Error {
[00:56:38]     Error {
[00:56:38]         line_num: 12,
[00:56:38]         kind: Some(
[00:56:38]             Error
[00:56:38]         ),
[00:56:38]         msg: "expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `pub`"
[00:56:38] ]
[00:56:38] 
[00:56:38] thread '[parse-fail] parse-fail/trait-pub-method.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:38] 
---
[00:56:38] test result: FAILED. 261 passed; 7 failed; 3 ignored; 0 measured; 0 filtered out
[00:56:38] 
[00:56:38] 
[00:56:38] 
[00:56:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:38] 
[00:56:38] 
[00:56:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:38] Build completed unsuccessfully in 0:14:27
[00:56:38] Build completed unsuccessfully in 0:14:27
[00:56:38] make: *** [check] Error 1
[00:56:38] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:330a7612
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
