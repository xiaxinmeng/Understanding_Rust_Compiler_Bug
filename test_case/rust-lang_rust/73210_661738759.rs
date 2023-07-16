
---- [codegen] codegen/lifetime_start_end.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-8/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll" "/checkout/src/test/codegen/lifetime_start_end.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/src/test/codegen/lifetime_start_end.rs:11:11: error: CHECK: expected string not found in input
// CHECK: [[S_a:%[0-9]+]] = bitcast i32* %a to i8*
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll:9:18: note: scanning from here
define void @test() unnamed_addr #0 {
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll:15:2: note: possible intended match here
 %0 = bitcast i32* %_1 to i8*
 ^
