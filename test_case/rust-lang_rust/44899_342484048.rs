llvm

; Function Attrs: nounwind
declare void @llvm.assume(i1)

; Function Attrs: nounwind uwtable
define i64 @icey(i1 zeroext) {
start:
  %. = select i1 %0, i64 -1, i64 -2
  %_4 = icmp eq i64 %., 0
  call void @llvm.assume(i1 %_4)

  ret i64 %.
}
