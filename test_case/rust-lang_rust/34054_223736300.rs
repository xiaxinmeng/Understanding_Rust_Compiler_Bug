

failures:

---- [codegen] codegen/stores.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: /usr/lib/llvm-3.7/bin/FileCheck -input-file=x86_64-unknown-linux-gnu/test/codegen/stores.ll /build/src/test/codegen/stores.rs
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/build/src/test/codegen/stores.rs:29:11: error: expected string not found in input
// CHECK: store i32 %{{.*}}, i32* %{{.*}}, align 1
          ^
x86_64-unknown-linux-gnu/test/codegen/stores.ll:8:35: note: scanning from here
define void @small_array_alignment([4 x i8]* dereferenceable(4), i32) unnamed_addr #0 {
                                  ^
x86_64-unknown-linux-gnu/test/codegen/stores.ll:15:2: note: possible intended match here
 store i32 %1, i32* %fn_ret_cast
 ^
/build/src/test/codegen/stores.rs:40:11: error: expected string not found in input
// CHECK: store i32 %{{.*}}, i32* %{{.*}}, align 1
          ^
x86_64-unknown-linux-gnu/test/codegen/stores.ll:44:36: note: scanning from here
define void @small_struct_alignment(%Bytes* dereferenceable(4), i32) unnamed_addr #0 {
                                   ^
x86_64-unknown-linux-gnu/test/codegen/stores.ll:51:2: note: possible intended match here
 store i32 %1, i32* %fn_ret_cast
 ^
