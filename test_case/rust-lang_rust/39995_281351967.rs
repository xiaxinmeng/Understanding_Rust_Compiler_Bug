
C:\projects\rust\src/test\codegen\function-arguments.rs:124:11: error: expected string not found in input
// CHECK: @trait_borrow(i8* nonnull, void (i8*)** nonnull)
          ^
C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\function-arguments.ll:266:62: note: scanning from here
define internal void @str(i8* noalias nonnull readonly, i64) unnamed_addr #1 {
                                                             ^
C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\function-arguments.ll:276:22: note: possible intended match here
define internal void @trait_borrow(i8* nonnull, void (i8*)** noalias nonnull readonly) unnamed_addr #1 {
