plain
[00:46:05] ..............................................................................i.....................
[00:46:10] ....................................................................................................
[00:46:16] ....................................................................................................
[00:46:22] ....................................................................................................
[00:46:26] ...........i.................iiiiiiiii...................................................
[00:46:26] 
[00:46:26] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:47:17] ..............................................................................i.....................
[00:47:22] ....................................................................................................
[00:47:26] ....................................................................................................
[00:47:32] ....................................................................................................
[00:47:37] ...........i.................iiiiiiiii...................................................
[00:47:37] 
[00:47:37]  finished in 70.207
[00:47:37] travis_fold:end:test_ui_nll

---
[00:57:06] ....................................................................................................
[00:57:11] ....................................................................................................
[00:57:17] ......................................................................................i.............
[00:57:23] ...............................................ii.iii...............................................
[00:57:28] ...............................................................F...............i....................
[00:57:38] ...............................................................................................i....
[00:57:42] ....................................................................................................
[00:57:48] ....................................................................................................
[00:57:53] ....................................................................................................
---
[00:59:03] failures:
[00:59:03] 
[00:59:03] ---- [compile-fail] compile-fail/feature-gate-panic-implementation.rs stdout ----
[00:59:03] 
[00:59:03] error: /checkout/src/test/compile-fail/feature-gate-panic-implementation.rs:18: expected error not found: #[panic_implementation] is an unstable feature (see issue #44489)
[00:59:03] 
[00:59:03] error: 0 unexpected errors found, 1 expected errors not found
[00:59:03] status: exit code: 101
[00:59:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/feature-gate-panic-implementation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-panic-implementation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-panic-implementation/auxiliary" "-A" "unused"
[00:59:03] not found errors (from test file): [
[00:59:03]     Error {
[00:59:03]         line_num: 18,
[00:59:03]         kind: Some(
[00:59:03]         ),
[00:59:03]         ),
[00:59:03]         msg: "#[panic_implementation] is an unstable feature (see issue #44489)"
[00:59:03] ]
[00:59:03] 
[00:59:03] thread '[compile-fail] compile-fail/feature-gate-panic-implementation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:59:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:59:03] 
[00:59:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:59:03] 
[00:59:03] 
[00:59:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:03] 
[00:59:03] 
[00:59:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:03] Build completed unsuccessfully in 0:15:16
[00:59:03] Build completed unsuccessfully in 0:15:16
[00:59:03] make: *** [check] Error 1
[00:59:03] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dd32d4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
