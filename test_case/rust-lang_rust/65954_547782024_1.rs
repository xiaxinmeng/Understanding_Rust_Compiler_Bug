llvm
; playground::foo
; Function Attrs: nonlazybind uwtable
define internal zeroext i1 @_ZN10playground3foo17h3b9711775736012fE() unnamed_addr #0 !dbg !5 {
start:
  ret i1 true, !dbg !10
}

; playground::bar
; Function Attrs: nonlazybind uwtable
define zeroext i1 @_ZN10playground3bar17hfc942ebdcf035a2cE() unnamed_addr #0 !dbg !11 {
start:
; call playground::foo
  %0 = call zeroext i1 @_ZN10playground3foo17h3b9711775736012fE(), !dbg !12
  br label %bb1, !dbg !12

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !13
}
