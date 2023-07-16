llvm
define void @_ZN10playground11TTTTTTTTTTT17h3759d258fafcd0c1E() unnamed_addr #2 !dbg !16 {
start:
; call playground::A::new
  call void @_ZN10playground1A3new17h4b221654e29fa3dfE(), !dbg !17
  br label %bb1, !dbg !17

bb1:                                              ; preds = %start
; call playground::B::new
  call void @_ZN10playground1B3new17h3a0060fa4c71b183E(), !dbg !18
  br label %bb2, !dbg !18

bb2:                                              ; preds = %bb1
  ret void, !dbg !20
}
