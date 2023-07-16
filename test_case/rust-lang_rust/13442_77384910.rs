 llvm
; Function Attrs: alwaysinline uwtable
define internal i64 @_ZN3bar20h4b0789288b21a387eaaE() unnamed_addr #0 {
entry-block:
  ret i64 5, !dbg !19
}

; Function Attrs: uwtable
define internal void @_ZN4main20h0007eb42fb7e75b0jaaE() unnamed_addr #1 {
entry-block:
  %0 = alloca i64
  %let = alloca i64
  %1 = call i64 @_ZN3bar20h4b0789288b21a387eaaE(), !dbg !20
  store i64 %1, i64* %0, !dbg !20
  %2 = load i64* %0, !dbg !20
  store i64 %2, i64* %let
  ret void, !dbg !22
}
...
!19 = !MDLocation(line: 4, scope: !4)
!20 = !MDLocation(line: 7, scope: !21)
