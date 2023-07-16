
[00:55:24] failures:
[00:55:24] 
[00:55:24] ---- [codegen] codegen/slice-init.rs stdout ----
[00:55:24] 	
[00:55:24] error: verification with 'FileCheck' failed
[00:55:24] status: exit code: 1
[00:55:24] command: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck -input-file=/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll /checkout/src/test/codegen/slice-init.rs
[00:55:24] stdout:
[00:55:24] ------------------------------------------
[00:55:24] 
[00:55:24] ------------------------------------------
[00:55:24] stderr:
[00:55:24] ------------------------------------------
[00:55:24] /checkout/src/test/codegen/slice-init.rs:36:12: error: expected string not found in input
[00:55:24]  // CHECK: call void @llvm.memset.p0i8.i{{[0-9]+}}(i8* {{.*}}, i8 7, i64 4
[00:55:24]            ^
[00:55:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll:81:24: note: scanning from here
[00:55:24] define void @byte_array() unnamed_addr #1 {
[00:55:24]                        ^
[00:55:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll:87:2: note: possible intended match here
[00:55:24]  call void @llvm.memset.p0i8.i32(i8* %1, i8 7, i32 4, i32 1, i1 false)
[00:55:24]  ^
[00:55:24] /checkout/src/test/codegen/slice-init.rs:52:12: error: expected string not found in input
[00:55:24]  // CHECK: call void @llvm.memset.p0i8.i{{[0-9]+}}(i8* {{.*}}, i8 {{.*}}, i64 4
[00:55:24]            ^
[00:55:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll:99:29: note: scanning from here
[00:55:24] define void @byte_enum_array() unnamed_addr #1 {
[00:55:24]                             ^
[00:55:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll:109:2: note: possible intended match here
[00:55:24]  call void @llvm.memset.p0i8.i32(i8* %2, i8 %1, i32 4, i32 1, i1 false)
[00:55:24]  ^
[00:55:24] /checkout/src/test/codegen/slice-init.rs:61:12: error: expected string not found in input
[00:55:24]  // CHECK: call void @llvm.memset.p0i8.i{{[0-9]+}}(i8* {{.*}}, i8 0, i64 16
[00:55:24]            ^
[00:55:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll:122:34: note: scanning from here
[00:55:24] define void @zeroed_integer_array() unnamed_addr #1 {
[00:55:24]                                  ^
[00:55:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init.ll:129:2: note: possible intended match here
[00:55:24]  call void @llvm.memset.p0i8.i32(i8* %2, i8 0, i32 16, i32 4, i1 false)
[00:55:24]  ^
[00:55:24] 
[00:55:24] ------------------------------------------
[00:55:24] 
[00:55:24] thread '[codegen] codegen/slice-init.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2499:8
[00:55:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:24] 
[00:55:24] 
[00:55:24] failures:
[00:55:24]     [codegen] codegen/slice-init.rs
[00:55:24] 
[00:55:24] test result: FAILED. 43 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
