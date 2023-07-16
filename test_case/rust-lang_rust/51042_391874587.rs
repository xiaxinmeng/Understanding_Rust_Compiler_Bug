plain
[00:50:29] ............................................i.......................................................
[00:50:33] ....................................................................................................
[00:50:36] ....................................................................................................
[00:50:40] ....................................................................................................
[00:50:43] .................................................................................F..................
[00:50:52] ....................................................................................................
[00:50:57] ....................................................................................................
[00:51:03] ..............................................................................i.....................
[00:51:08] ......................................................i.............................................
---
[00:51:29] failures:
[00:51:29] 
[00:51:29] ---- [ui] ui/feature-gate-trivial_bounds.rs stdout ----
[00:51:29] 
[00:51:29] error: /checkout/src/test/ui/feature-gate-trivial_bounds.rs:31: expected error not found: 
[00:51:29] 
[00:51:29] error: /checkout/src/test/ui/feature-gate-trivial_bounds.rs:62: expected error not found: 
[00:51:29] 
[00:51:29] error: 0 unexpected errors found, 2 expected errors not found
[00:51:29] status: exit code: 101
[00:51:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-trivial_bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds/auxiliary" "-A" "unused"
[00:51:29] not found errors (from test file): [
[00:51:29]     Error {
[00:51:29]         line_num: 31,
[00:51:29]         kind: Some(
[00:51:29]         ),
[00:51:29]         ),
[00:51:29]         msg: ""
[00:51:29]     Error {
[00:51:29]         line_num: 62,
[00:51:29]         kind: Some(
[00:51:29]             Error
[00:51:29]             Error
[00:51:29]         ),
[00:51:29]         msg: ""
[00:51:29] ]
[00:51:29] 
[00:51:29] thread '[ui] ui/feature-gate-trivial_bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:51:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:29] 
[00:51:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:29] 
[00:51:29] 
[00:51:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:29] 
[00:51:29] 
[00:51:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:29] Build completed unsuccessfully in 0:02:48
[00:51:29] Build completed unsuccessfully in 0:02:48
[00:51:29] Makefile:58: recipe for target 'check' failed
[00:51:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:069e7e16
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
