
[00:58:30] ---- [codegen] codegen/repr-transparent.rs stdout ----
[00:58:30] 	
[00:58:30] error: verification with 'FileCheck' failed
[00:58:30] status: exit code: 1
[00:58:30] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll" "/checkout/src/test/codegen/repr-transparent.rs"
[00:58:30] stdout:
[00:58:30] ------------------------------------------
[00:58:30] 
[00:58:30] ------------------------------------------
[00:58:30] stderr:
[00:58:30] ------------------------------------------
[00:58:30] /checkout/src/test/codegen/repr-transparent.rs:148:11: error: expected string not found in input
[00:58:30] // CHECK: define i32 @test_Rgb8Wrap(i32
[00:58:30]           ^
[00:58:30] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll:120:44: note: scanning from here
[00:58:30] define float @test_Projection(float %arg0) unnamed_addr #0 {
[00:58:30]                                            ^
[00:58:30] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll:129:1: note: possible intended match here
[00:58:30] define i24 @test_Rgb8Wrap(i24) unnamed_addr #0 {
[00:58:30] ^
[00:58:30] 
[00:58:30] ------------------------------------------
[00:58:30] 
[00:58:30] thread '[codegen] codegen/repr-transparent.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[00:58:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:30] 
[00:58:30] 
[00:58:30] failures:
[00:58:30]     [codegen] codegen/repr-transparent.rs
[00:58:30] 
[00:58:30] test result: [31mFAILED(B[m. 50 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out
