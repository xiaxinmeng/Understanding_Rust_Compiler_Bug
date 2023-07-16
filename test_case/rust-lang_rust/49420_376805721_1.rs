llvm
; Function Attrs: norecurse nounwind readnone uwtable
define { i32, i32 } @id_result(i32, i32) unnamed_addr #0 {
start:
  %switch = icmp ne i32 %0, 0
  %. = zext i1 %switch to i32
  %2 = insertvalue { i32, i32 } undef, i32 %., 0
  %3 = insertvalue { i32, i32 } %2, i32 %1, 1
  ret { i32, i32 } %3
}
