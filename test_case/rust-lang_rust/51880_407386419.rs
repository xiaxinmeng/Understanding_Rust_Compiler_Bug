plain
[00:50:14] ....................................................................................................
[00:50:17] ....................................................................................................
[00:50:19] .......................................i............................................................
[00:50:23] ....................................................................................................
[00:50:25] .........F..........................................................................................
[00:50:30] .....................................................................................
[00:50:30] failures:
[00:50:30] 
[00:50:30] ---- [compile-fail] compile-fail/trait-object-vs-lifetime.rs stdout ----
[00:50:30] ---- [compile-fail] compile-fail/trait-object-vs-lifetime.rs stdout ----
[00:50:30] 
[00:50:30] error: /checkout/src/test/compile-fail/trait-object-vs-lifetime.rs:26: unexpected error: '26:14: 26:23: at least one non-builtin trait is required for an object type [E0224]'
[00:50:30] error: 1 unexpected errors found, 0 expected errors not found
[00:50:30] status: exit code: 1
[00:50:30] status: exit code: 1
[00:50:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/trait-object-vs-lifetime.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trait-object-vs-lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/trait-object-vs-lifetime/auxiliary" "-A" "unused"
[00:50:30]     Error {
[00:50:30]         line_num: 26,
[00:50:30]         kind: Some(
[00:50:30]             Error
[00:50:30]             Error
[00:50:30]         ),
[00:50:30]         msg: "26:14: 26:23: at least one non-builtin trait is required for an object type [E0224]"
[00:50:30] ]
[00:50:30] 
[00:50:30] thread '[compile-fail] compile-fail/trait-object-vs-lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1285:13
[00:50:30] 
---
[00:50:30] 
[00:50:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:30] 
[00:50:30] 
[00:50:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:30] 
[00:50:30] 
[00:50:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:30] Build completed unsuccessfully in 0:09:02
[00:50:30] Build completed unsuccessfully in 0:09:02
[00:50:30] make: *** [check] Error 1
[00:50:30] Makefile:58: recipe for target 'check' failed
