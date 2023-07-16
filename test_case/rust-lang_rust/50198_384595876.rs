plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:53] 
[01:07:53] running 50 tests
[01:08:16] ...F..............................................
[01:08:16] 
[01:08:16] ---- [mir-opt] mir-opt/deaggregator_test.rs stdout ----
[01:08:16] ---- [mir-opt] mir-opt/deaggregator_test.rs stdout ----
[01:08:16]  thread '[mir-opt] mir-opt/deaggregator_test.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:08:16] Expected Line: "    _0 = Baz { x: move _2, y: const 0f32, z: const false };"
[01:08:16] Expected:
[01:08:16] ... (elided)
[01:08:16] bb0: {
[01:08:16] ... (elided)
[01:08:16]     _2 = _1;
[01:08:16] ... (elided)
[01:08:16]     _0 = Baz { x: move _2, y: const 0f32, z: const false };
[01:08:16] ... (elided)
[01:08:16]     return;
[01:08:16] }
[01:08:16] Actual:
[01:08:16] fn bar(_1: usize) -> Baz{
[01:08:16]     let mut _0: Baz;
[01:08:16]     let mut _2: usize;
[01:08:16]     bb0: {                              
[01:08:16]         StorageLive(_2);
[01:08:16]         _2 = _1;
[01:08:16]         _0 = Baz { x: move _2, y: const 0, z: const false };
[01:08:16]         StorageDead(_2);
[01:08:16]         return;
[01:08:16]     bb1: {
[01:08:16]         resume;
[01:08:16]     }
[01:08:16]     }
[01:08:16] }', tools/compiletest/src/runtest.rs:2733:13
[01:08:16] 
[01:08:16] 
[01:08:16] failures:
[01:08:16]     [mir-opt] mir-opt/deaggregator_test.rs
[01:08:16]     [mir-opt] mir-opt/deaggregator_test.rs
[01:08:16] 
[01:08:16] test result: FAILED. 49 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:16] 
[01:08:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:08:16] 
[01:08:16] 
[01:08:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:16] 
[01:08:16] 
[01:08:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:16] Build completed unsuccessfully in 0:19:54
[01:08:16] Build completed unsuccessfully in 0:19:54
[01:08:16] make: *** [check] Error 1
[01:08:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0813ba91
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
