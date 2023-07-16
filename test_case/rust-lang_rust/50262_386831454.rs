plain
[01:04:32] ....................................................................................................
[01:04:41] .......................................................................i............................
[01:04:51] ................i...................................................................................
[01:05:00] ....................................................................................................
[01:05:08] .............................................................F......................................
[01:05:18] ..........
[01:05:18] failures:
[01:05:18] 
[01:05:18] ---- [compile-fail] compile-fail/unreachable-try-pattern.rs stdout ----
[01:05:18] ---- [compile-fail] compile-fail/unreachable-try-pattern.rs stdout ----
[01:05:18]  
[01:05:18] error: /checkout/src/test/compile-fail/unreachable-try-pattern.rs:19: expected warning not found: unreachable expression
[01:05:18] 
[01:05:18] error: 0 unexpected errors found, 1 expected errors not found
[01:05:18] status: exit code: 101
[01:05:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/unreachable-try-pattern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unreachable-try-pattern.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unreachable-try-pattern.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:05:18] not found errors (from test file): [
[01:05:18]     Error {
[01:05:18]         line_num: 19,
[01:05:18]         kind: Some(
[01:05:18]         ),
[01:05:18]         ),
[01:05:18]         msg: "unreachable expression"
[01:05:18] ]
[01:05:18] 
[01:05:18] thread '[compile-fail] compile-fail/unreachable-try-pattern.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1279:13
[01:05:18] 
---
[01:05:18] 
[01:05:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:489:22
[01:05:18] 
[01:05:18] 
[01:05:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:18] 
[01:05:18] 
[01:05:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:18] Build completed unsuccessfully in 0:17:42
[01:05:18] Build completed unsuccessfully in 0:17:42
[01:05:18] make: *** [check] Error 1
[01:05:18] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00144364
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
