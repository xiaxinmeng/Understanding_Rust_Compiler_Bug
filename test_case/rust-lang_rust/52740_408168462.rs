plain
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:36] 
[00:51:36] running 271 tests
[00:51:38] ...........................i.......................F................................................
[00:51:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:40] ...............i.......................................................
[00:51:40] failures:
[00:51:40] 
[00:51:40] 
[00:51:40] ---- [parse-fail] parse-fail/extern-foreign-crate.rs stdout ----
[00:51:40] 
[00:51:40] error: /checkout/src/test/parse-fail/extern-foreign-crate.rs:16: unexpected error: '16:18: 16:19: expected one of `-`, `;`, or `as`, found `{`'
[00:51:40] 
[00:51:40] error: /checkout/src/test/parse-fail/extern-foreign-crate.rs:16: expected error not found: expected one of `;` or `as`, found `{`
[00:51:40] error: 1 unexpected errors found, 1 expected errors not found
[00:51:40] status: exit code: 1
[00:51:40] status: exit code: 1
[00:51:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/extern-foreign-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/extern-foreign-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/extern-foreign-crate/auxiliary"
[00:51:40]     Error {
[00:51:40]         line_num: 16,
[00:51:40]         kind: Some(
[00:51:40]             Error
[00:51:40]             Error
[00:51:40]         ),
[00:51:40]         msg: "16:18: 16:19: expected one of `-`, `;`, or `as`, found `{`"
[00:51:40] ]
[00:51:40] 
[00:51:40] not found errors (from test file): [
[00:51:40]     Error {
[00:51:40]     Error {
[00:51:40]         line_num: 16,
[00:51:40]         kind: Some(
[00:51:40]             Error
[00:51:40]         ),
[00:51:40]         msg: "expected one of `;` or `as`, found `{`"
[00:51:40] ]
[00:51:40] 
[00:51:40] thread '[parse-fail] parse-fail/extern-foreign-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1285:13
[00:51:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:40] test result: FAILED. 267 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[00:51:40] 
[00:51:40] 
[00:51:40] 
[00:51:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:40] 
[00:51:40] 
[00:51:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:40] Build completed unsuccessfully in 0:09:18
[00:51:40] Build completed unsuccessfully in 0:09:18
[00:51:40] Makefile:58: recipe for target 'check' failed
[00:51:40] make: *** [check] Error 1
122144 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121708 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
111288 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
107600 ./src/llvm/test/CodeGen
