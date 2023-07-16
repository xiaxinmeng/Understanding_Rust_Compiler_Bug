
[01:00:07] ---- [codegen] codegen/repr-transparent.rs stdout ----
[01:00:07] 	
[01:00:07] error: verification with 'FileCheck' failed
[01:00:07] status: exit code: 1
[01:00:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll" "/checkout/src/test/codegen/repr-transparent.rs"
[01:00:07] stdout:
[01:00:07] ------------------------------------------
[01:00:07] 
[01:00:07] ------------------------------------------
[01:00:07] stderr:
[01:00:07] ------------------------------------------
[01:00:07] /checkout/src/test/codegen/repr-transparent.rs:160:11: error: expected string not found in input
[01:00:07] // CHECK: define i32 @test_Rgb8Wrap(i32 %arg0)
[01:00:07]           ^
[01:00:07] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll:142:135: note: scanning from here
[01:00:07] define void @test_BigW(%BigW* noalias nocapture sret dereferenceable(128), %BigW* byval noalias nocapture dereferenceable(128) %arg0) unnamed_addr #0 {
[01:00:07]                                                                                                                                       ^
[01:00:07] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll:151:1: note: possible intended match here
[01:00:07] define void @test_Rgb8Wrap(%Rgb8Wrap* noalias nocapture sret dereferenceable(3), %Rgb8Wrap* byval noalias nocapture dereferenceable(3) %arg0) unnamed_addr #0 {
[01:00:07] ^
[01:00:07] 
[01:00:07] ------------------------------------------
[01:00:07] 
[01:00:07] thread '[codegen] codegen/repr-transparent.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2783:9
[01:00:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:07] 
[01:00:07] 
[01:00:07] failures:
[01:00:07]     [codegen] codegen/repr-transparent.rs
[01:00:07] 
[01:00:07] test result: [31mFAILED(B[m. 54 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
