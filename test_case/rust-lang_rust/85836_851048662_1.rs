llvm
; playground::foo
; Function Attrs: nonlazybind uwtable
define i32 @_ZN10playground3foo17hb9992b01cb0c258eE() unnamed_addr #0 !dbg !6 {
start:
  %0 = alloca i32, align 4
  br i1 true, label %bb1, label %bb2, !dbg !15

bb2:                                              ; preds = %start
  store i32 5, i32* %0, align 4, !dbg !16
  br label %bb3, !dbg !15

bb1:                                              ; preds = %start
  store i32 3, i32* %0, align 4, !dbg !17
  br label %bb3, !dbg !15

bb3:                                              ; preds = %bb2, %bb1
  %1 = load i32, i32* %0, align 4, !dbg !18
  ret i32 %1, !dbg !18
}
