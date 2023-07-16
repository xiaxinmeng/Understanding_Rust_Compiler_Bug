plain
[01:00:40] ....................................................................................................
[01:00:47] ........................................................................i...........................
[01:00:56] .................i..................................................................................
[01:01:05] ....................................................................................................
[01:01:14] ..............................................................................................F.....
[01:01:23] failures:
[01:01:23] 
[01:01:23] ---- [compile-fail] compile-fail/unused-result.rs stdout ----
[01:01:23]  
[01:01:23]  
[01:01:23] error: /checkout/src/test/compile-fail/unused-result.rs:11: unexpected note: '11:25: 11:40: lint level defined here'
[01:01:23] 
[01:01:23] error: /checkout/src/test/compile-fail/unused-result.rs:11: unexpected note: '11:9: 11:23: lint level defined here'
[01:01:23] 
[01:01:23] error: 2 unexpected errors found, 0 expected errors not found
[01:01:23] status: exit code: 101
[01:01:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/unused-result.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unused-result.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unused-result.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:01:23] unexpected errors (from JSON output): [
[01:01:23]     Error {
[01:01:23]         line_num: 11,
[01:01:23]         kind: Some(
[01:01:23]         ),
[01:01:23]         ),
[01:01:23]         msg: "11:25: 11:40: lint level defined here"
[01:01:23]     Error {
[01:01:23]         line_num: 11,
[01:01:23]         kind: Some(
[01:01:23]             Note
[01:01:23]             Note
[01:01:23]         ),
[01:01:23]         msg: "11:9: 11:23: lint level defined here"
[01:01:23] ]
[01:01:23] 
[01:01:23] thread '[compile-fail] compile-fail/unused-result.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:01:23] 
---
[01:01:23] 
[01:01:23] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:01:23] 
[01:01:23] 
[01:01:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:23] 
[01:01:23] 
[01:01:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:23] Build completed unsuccessfully in 0:16:53
[01:01:23] Build completed unsuccessfully in 0:16:53
[01:01:23] Makefile:58: recipe for target 'check' failed
[01:01:23] make: *** [check] Error 1
