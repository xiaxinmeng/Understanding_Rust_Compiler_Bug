 llvm
; Function Attrs: uwtable
define internal void @_ZN4main20h4bedfa0b9eaa7039jaaE() unnamed_addr #0 {
entry-block:
  %0 = alloca i32
  %let = alloca i32
  store i32 5, i32* %0, !dbg !18 ; <=== no function call 
  %1 = load i32* %0, !dbg !18
  store i32 %1, i32* %let
  ret void, !dbg !20
}
