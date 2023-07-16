
---- [codegen] codegen/vec-in-place.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll" "/checkout/src/test/codegen/vec-in-place.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/src/test/codegen/vec-in-place.rs:10:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:167:2: note: found here
 call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2), !noalias !20
 ^~~~

------------------------------------------
