plain
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:52] 
[01:02:52] running 78 tests
[01:02:56] i...ii..ii....i...i..........ii.........iii......i..i..Fi...ii..i..i...ii.....
[01:02:56] 
[01:02:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:02:56] ---- [codegen] codegen/packed.rs stdout ----
[01:02:56] 
[01:02:56] 
[01:02:56] error: verification with 'FileCheck' failed
[01:02:56] status: exit code: 1
[01:02:56] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll" "/checkout/src/test/codegen/packed.rs"
[01:02:56] ------------------------------------------
[01:02:56] 
[01:02:56] ------------------------------------------
[01:02:56] stderr:
[01:02:56] stderr:
[01:02:56] ------------------------------------------
[01:02:56] /checkout/src/test/codegen/packed.rs:65:11: error: expected string not found in input
[01:02:56] // CHECK: call void %{{.*}}(%Array* noalias nocapture sret dereferenceable(32) [[ALLOCA]])
[01:02:56]           ^
[01:02:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll:39:21: note: scanning from here
[01:02:56]  %_2 = alloca %Array, align 4
[01:02:56]                     ^
[01:02:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll:39:21: note: with variable "ALLOCA" equal to "%_2"
[01:02:56]  %_2 = alloca %Array, align 4
[01:02:56]                     ^
[01:02:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll:42:2: note: possible intended match here
[01:02:56]  call void %f(%Array* noalias nocapture sret align 4 dereferenceable(32) %_2)
[01:02:56]  ^
[01:02:56] /checkout/src/test/codegen/packed.rs:77:11: error: expected string not found in input
[01:02:56] // CHECK: call void %{{.*}}(%Array* noalias nocapture sret dereferenceable(32) [[ALLOCA]])
[01:02:56]           ^
[01:02:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll:60:21: note: scanning from here
[01:02:56]  %_2 = alloca %Array, align 4
[01:02:56]                     ^
[01:02:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll:60:21: note: with variable "ALLOCA" equal to "%_2"
[01:02:56]  %_2 = alloca %Array, align 4
[01:02:56]                     ^
[01:02:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/packed/packed.ll:63:2: note: possible intended match here
[01:02:56]  call void %f(%Array* noalias nocapture sret align 4 dereferenceable(32) %_2)
[01:02:56]  ^
[01:02:56] ------------------------------------------
[01:02:56] 
[01:02:56] thread '[codegen] codegen/packed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3042:9
[01:02:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:02:56] test result: FAILED. 56 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out
[01:02:56] 
[01:02:56] 
[01:02:56] 
[01:02:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:56] 
[01:02:56] 
[01:02:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:56] Build completed unsuccessfully in 0:15:29
[01:02:56] Build completed unsuccessfully in 0:15:29
[01:02:56] Makefile:58: recipe for target 'check' failed
[01:02:56] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00bc3898
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
