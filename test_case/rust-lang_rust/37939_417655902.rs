llvm
; Function Attrs: noinline norecurse nounwind readnone uwtable
define { i64, i64 } @try_op(i64, i64) unnamed_addr #2 {
  %3 = tail call { i64, i64 } @try_macro(i64 %0, i64 %1) #2
  ret { i64, i64 } %3
}
