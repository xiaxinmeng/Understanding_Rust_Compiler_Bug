plain
[00:56:21] ..........................................................................i.........................
[00:56:28] ....................................................................................................
[00:56:35] ....................................................................................................
[00:56:42] ....................................................................................................
[00:56:48] ......i...........F......iiiiiiiii...................................................
[00:56:48] 
[00:56:48] ---- [ui] ui/trait-object-auto-dedup-in-impl.rs stdout ----
[00:56:48] 
[00:56:48] 
[00:56:48] error: /checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs:24: unexpected error: '24:5: 24:40: duplicate definitions with name `test` [E0592]'
[00:56:48] 
[00:56:48] error: /checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs:24: expected error not found: duplicate definitions for `test`
[00:56:48] 
[00:56:48] error: 1 unexpected errors found, 1 expected errors not found
[00:56:48] status: exit code: 101
[00:56:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-object-auto-dedup-in-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-object-auto-dedup-in-impl/auxiliary" "-A" "unused"
[00:56:48] unexpected errors (from JSON output): [
[00:56:48]     Error {
[00:56:48]         line_num: 24,
[00:56:48]         kind: Some(
[00:56:48]         ),
[00:56:48]         ),
[00:56:48]         msg: "24:5: 24:40: duplicate definitions with name `test` [E0592]"
[00:56:48] ]
[00:56:48] 
[00:56:48] not found errors (from test file): [
[00:56:48]     Error {
[00:56:48]     Error {
[00:56:48]         line_num: 24,
[00:56:48]         kind: Some(
[00:56:48]             Error
[00:56:48]         ),
[00:56:48]         msg: "duplicate definitions for `test`"
[00:56:48] ]
[00:56:48] 
[00:56:48] thread '[ui] ui/trait-object-auto-dedup-in-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:56:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:48] 
[00:56:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:48] 
[00:56:48] 
[00:56:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:48] 
[00:56:48] 
[00:56:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:48] Build completed unsuccessfully in 0:03:23
[00:56:48] Build completed unsuccessfully in 0:03:23
[00:56:48] Makefile:58: recipe for target 'check' failed
[00:56:48] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08951c50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
