plain
[01:00:33] ....................................................................................................
[01:00:41] .........................................................................i..........................
[01:00:50] ..................i.................................................................................
[01:01:00] ....................................................................................................
[01:01:08] ......................................F.............................................................
[01:01:18] .........
[01:01:18] failures:
[01:01:18] 
[01:01:18] ---- [compile-fail] compile-fail/uninhabited-matches-feature-gated.rs stdout ----
[01:01:18] ---- [compile-fail] compile-fail/uninhabited-matches-feature-gated.rs stdout ----
[01:01:18]  
[01:01:18] error: /checkout/src/test/compile-fail/uninhabited-matches-feature-gated.rs:26: expected error not found: non-exhaustive
[01:01:18] 
[01:01:18] error: 0 unexpected errors found, 1 expected errors not found
[01:01:18] status: exit code: 101
[01:01:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/uninhabited-matches-feature-gated.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/uninhabited-matches-feature-gated.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/uninhabited-matches-feature-gated.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:01:18] not found errors (from test file): [
[01:01:18]     Error {
[01:01:18]         line_num: 26,
[01:01:18]         kind: Some(
[01:01:18]             Error
[01:01:18]         ),
[01:01:18]         msg: "non-exhaustive"
[01:01:18] ]
[01:01:18] 
[01:01:18] thread '[compile-fail] compile-fail/uninhabited-matches-feature-gated.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[01:01:18] 
---
[01:01:18] 
[01:01:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:01:18] 
[01:01:18] 
[01:01:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:18] 
[01:01:18] 
[01:01:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:18] Build completed unsuccessfully in 0:17:34
[01:01:18] Build completed unsuccessfully in 0:17:34
[01:01:18] Makefile:58: recipe for target 'check' failed
[01:01:18] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17d94461
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
