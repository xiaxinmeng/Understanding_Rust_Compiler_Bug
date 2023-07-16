
C:\projects\rust\src/test\codegen\function-arguments.rs:56:11: error: CHECK: expected string not found in input
// CHECK: @mutable_unsafe_borrow(i16* noalias dereferenceable(2) %arg0)
          ^
C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\function-arguments\function-arguments.ll:272:59: note: scanning from here
define void @unsafe_borrow(i16* dereferenceable(2) %arg0) unnamed_addr #0 {
                                                          ^
C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\function-arguments\function-arguments.ll:278:5: note: possible intended match here
define void @mutable_unsafe_borrow(i16* dereferenceable(2) %arg0) unnamed_addr #0 {
    ^
