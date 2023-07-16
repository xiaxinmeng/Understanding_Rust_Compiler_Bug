
---- [codegen] codegen\function-arguments.rs stdout ----

...

stderr:
------------------------------------------
C:\projects\rust\src/test\codegen\function-arguments.rs:123:11: error: expected string not found in input
// CHECK: @trait_borrow({}* nonnull %arg0.0, [4 x [[USIZE]]]* noalias readonly dereferenceable({{.*}}) %arg0.1)
          ^
C:\projects\rust\build\i686-pc-windows-msvc\test\codegen\function-arguments\function-arguments.ll:364:75: note: scanning from here
define void @str([0 x i8]* noalias nonnull readonly %arg0.0, i32 %arg0.1) unnamed_addr #0 {
                                                                          ^
C:\projects\rust\build\i686-pc-windows-msvc\test\codegen\function-arguments\function-arguments.ll:364:75: note: with variable "USIZE" equal to "i32"
define void @str([0 x i8]* noalias nonnull readonly %arg0.0, i32 %arg0.1) unnamed_addr #0 {
                                                                          ^
C:\projects\rust\build\i686-pc-windows-msvc\test\codegen\function-arguments\function-arguments.ll:370:2: note: possible intended match here
define void @trait_borrow({}* nonnull %arg0.0, [3 x i32]* noalias readonly dereferenceable(12) %arg0.1) unnamed_addr #0 {
 ^
------------------------------------------
