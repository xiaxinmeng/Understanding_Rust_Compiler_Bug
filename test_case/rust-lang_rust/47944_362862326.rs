
[01:02:09] failures:
[01:02:09] 
[01:02:09] ---- [codegen] codegen/repeat-trusted-len.rs stdout ----
[01:02:09] 	
[01:02:09] error: verification with 'FileCheck' failed
[01:02:09] status: exit code: 1
[01:02:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll" "/checkout/src/test/codegen/repeat-trusted-len.rs"
[01:02:09] stdout:
[01:02:09] ------------------------------------------
[01:02:09] 
[01:02:09] ------------------------------------------
[01:02:09] stderr:
[01:02:09] ------------------------------------------
[01:02:09] /checkout/src/test/codegen/repeat-trusted-len.rs:28:11: error: expected string not found in input
[01:02:09] // CHECK: %[[SPLATINSERT:.*]] = insertelement <{{[0-9]+}} x i8> undef, i8 %{{.*}}, i32 0
[01:02:09]           ^
[01:02:09] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll:171:37: note: scanning from here
[01:02:09] define void @range_from_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:02:09]                                     ^
[01:02:09] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll:197:2: note: possible intended match here
[01:02:09]  %9 = getelementptr inbounds [0 x i8], [0 x i8]* %7, i32 0, i32 %8
[01:02:09]  ^
[01:02:09] 
[01:02:09] ------------------------------------------
