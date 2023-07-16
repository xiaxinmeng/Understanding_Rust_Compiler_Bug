
[00:51:57] ---- [codegen] codegen/fastcall-inreg.rs stdout ----
[00:51:57] 	
[00:51:57] error: verification with 'FileCheck' failed
[00:51:57] status: exit code: 1
[00:51:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg.ll" "/checkout/src/test/codegen/fastcall-inreg.rs"
[00:51:57] stdout:
[00:51:57] ------------------------------------------
[00:51:57] 
[00:51:57] ------------------------------------------
[00:51:57] stderr:
[00:51:57] ------------------------------------------
[00:51:57] /checkout/src/test/codegen/fastcall-inreg.rs:63:12: error: expected string not found in input
[00:51:57]  // CHECK: @f1(i32 inreg %arg0, i32 inreg %arg1, i32 %arg2)
[00:51:57]            ^
[00:51:57] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg.ll:1:1: note: scanning from here
[00:51:57] ; ModuleID = 'fastcall_inreg0-8787f43e282added376259c1adb08b80.rs'
[00:51:57] ^
[00:51:57] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg.ll:3:35: note: possible intended match here
[00:51:57] target datalayout = "e-m:e-p:32:32-f64:32:64-f80:32-n8:16:32-S128"
[00:51:57]                                   ^
[00:51:57] 
[00:51:57] ------------------------------------------
[00:51:57] 
[00:51:57] thread '[codegen] codegen/fastcall-inreg.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2498:8
[00:51:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:57] 
[00:51:57] 
[00:51:57] failures:
[00:51:57]     [codegen] codegen/fastcall-inreg.rs
[00:51:57] 
[00:51:57] test result: [31mFAILED(B[m. 47 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
