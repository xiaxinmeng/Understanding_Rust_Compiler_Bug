llvm
; ...
; playground::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN10playground4main17h37f684608cd1371bE() unnamed_addr #0 !dbg !120 {
start:
  call void @func(i32 100), !dbg !122
  br label %bb1, !dbg !122

bb1:                                              ; preds = %start
  call void bitcast (void (i32)* @func to void ()*)(), !dbg !124
  br label %bb2, !dbg !124

bb2:                                              ; preds = %bb1
  ret void, !dbg !125
}
; ...
