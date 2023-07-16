plain
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:02] 
[01:00:02] running 93 tests
heckout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic-generic-scatter.stage2-x86_64-unknown-linux-gnu.aux" "--emit=llvm-ir"
[01:00:08] ------------------------------------------
[01:00:08] 
[01:00:08] ------------------------------------------
[01:00:08] stderr:
[01:00:08] stderr:
[01:00:08] ------------------------------------------
[01:00:08] Intrinsic name not mangled correctly for type arguments! Should be: llvm.masked.scatter.v2f32
[01:00:08] void (<2 x float>, <2 x float*>, i32, <2 x i1>)* @llvm.masked.scatter.v2f32.v2p0f32
[01:00:08] LLVM ERROR: Broken function found, compilation aborted!
[01:00:08] ------------------------------------------
[01:00:08] 
[01:00:08] 
[01:00:08] thread '[codegen] codegen/simd-intrinsic-generic-scatter.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:00:08] 
[01:00:08] failures:
[01:00:08] failures:
[01:00:08]     [codegen] codegen/simd-intrinsic-generic-gather.rs
[01:00:08]     [codegen] codegen/simd-intrinsic-generic-scatter.rs
[01:00:08] test result: FAILED. 71 passed; 2 failed; 20 ignored; 0 measured; 0 filtered out
[01:00:08] 
[01:00:08] 
[01:00:08] 
[01:00:08] 
[01:00:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:08] 
[01:00:08] 
[01:00:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:08] Build completed unsuccessfully in 0:17:35
[01:00:08] Build completed unsuccessfully in 0:17:35
[01:00:08] Makefile:58: recipe for target 'check' failed
[01:00:08] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b30d312
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
