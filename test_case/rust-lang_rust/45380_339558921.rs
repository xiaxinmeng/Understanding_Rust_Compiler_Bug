
[00:52:29] failures:
[00:52:29] 
[00:52:29] ---- [codegen] codegen/fastcall-inreg.rs stdout ----
[00:52:29] 	
[00:52:29] error: verification with 'FileCheck' failed
[00:52:29] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329:21
[00:52:29] status: exit code: 1
[00:52:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg.ll" "/checkout/src/test/codegen/fastcall-inreg.rs"
[00:52:29] stdout:
[00:52:29] ------------------------------------------
[00:52:29] 
[00:52:29] ------------------------------------------
[00:52:29] stderr:
[00:52:29] ------------------------------------------
[00:52:29] /checkout/src/test/codegen/fastcall-inreg.rs:63:12: error: expected string not found in input
[00:52:29]  // CHECK: @f1(i32 inreg, i32 inreg, i32)
[00:52:29]            ^
[00:52:29] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg.ll:1:1: note: scanning from here
[00:52:29] ; ModuleID = 'fastcall_inreg0-8787f43e282added376259c1adb08b80.rs'
[00:52:29] ^
[00:52:29] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg.ll:7:37: note: possible intended match here
[00:52:29] define internal x86_fastcallcc void @f1(i32 inreg %arg0, i32 inreg %arg1, i32 %arg2) unnamed_addr #0 {
[00:52:29]                                     ^
[00:52:29] 
[00:52:29] ------------------------------------------
[00:52:29] 
[00:52:29] thread '[codegen] codegen/fastcall-inreg.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2485:8
[00:52:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:29] 
[00:52:29] 
[00:52:29] failures:
[00:52:29]     [codegen] codegen/fastcall-inreg.rs
[00:52:29] 
[00:52:29] test result: [31mFAILED(B[m. 47 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:52:29] 
[00:52:29] 
[00:52:29] 
[00:52:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "codegen" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "4.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:29] expected success, got: exit code: 101
[00:52:29] 
[00:52:29] 
[00:52:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i686-unknown-linux-musl --target i586-unknown-linux-gnu
[00:52:29] Build completed unsuccessfully in 0:50:29
