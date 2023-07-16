
------------------------------------------
C:\projects\rust\src/test\codegen\union-abi.rs:66:11: error: CHECK: expected string not found in input
// CHECK: define i32 @test_UnionF32U32(i32)
          ^
C:\projects\rust\build\i686-pc-windows-msvc\test\codegen\union-abi\union-abi.ll:57:45: note: scanning from here
define float @test_UnionF32F32(float %arg0) unnamed_addr #0 {
                                            ^
C:\projects\rust\build\i686-pc-windows-msvc\test\codegen\union-abi\union-abi.ll:66:1: note: possible intended match here
define i32 @test_UnionF32U32(%UnionF32U32* byval noalias nocapture dereferenceable(4) %arg0) unnamed_addr #0 {
^
------------------------------------------
