
[01:00:21] stderr:
[01:00:21] ------------------------------------------
[01:00:21] /checkout/src/test/codegen/repr-transparent.rs:160:11: error: expected string not found in input
[01:00:21] // CHECK: define i32 @test_Rgb8Wrap(i32 %arg0)
[01:00:21]           ^
[01:00:21] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll:142:135: note: scanning from here
[01:00:21] define void @test_BigW(%BigW* noalias nocapture sret dereferenceable(128), %BigW* byval noalias nocapture dereferenceable(128) %arg0) unnamed_addr #0 {
[01:00:21]                                                                                                                                       ^
[01:00:21] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent.ll:151:1: note: possible intended match here
[01:00:21] define i32 @test_Rgb8Wrap(i32) unnamed_addr #0 {
[01:00:21] ^
[01:00:21] 
[01:00:21] ------------------------------------------
