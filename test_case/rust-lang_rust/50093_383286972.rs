plain
[01:21:24] failures:
[01:21:24] 
[01:21:24] ---- [codegen] codegen/nounwind.rs stdout ----
[01:21:24]  
[01:21:24] error: verification with 'FileCheck' failed
[01:21:24] status: exit code: 1
[01:21:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/nounwind.ll" "/checkout/src/test/codegen/nounwind.rs"
[01:21:24] ------------------------------------------
[01:21:24] 
[01:21:24] ------------------------------------------
[01:21:24] stderr:
[01:21:24] stderr:
[01:21:24] ------------------------------------------
[01:21:24] /checkout/src/test/codegen/nounwind.rs:23:11: error: expected string not found in input
[01:21:24] // CHECK: @bar() unnamed_addr #0
[01:21:24]           ^
[01:21:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/nounwind.ll:7:36: note: scanning from here
[01:21:24] define void @foo() unnamed_addr #0 {
[01:21:24]                                    ^
[01:21:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/nounwind.ll:17:14: note: possible intended match here
[01:21:24] declare void @bar() unnamed_addr #1
[01:21:24] 
[01:21:24] ------------------------------------------
[01:21:24] 
[01:21:24] thread '[codegen] codegen/nounwind.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2938:9
---
[01:21:24] test result: FAILED. 56 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out
[01:21:24] 
[01:21:24] 
[01:21:24] 
[01:21:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "codegen" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:21:24] 
[01:21:24] 
[01:21:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:21:24] Build completed unsuccessfully in 1:08:34
