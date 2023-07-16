llvm
define i8 @two_valued(i8) unnamed_addr #0 {
start:
  %1 = icmp ult i8 %0, 2
  tail call void @llvm.assume(i1 %1)
  ret i8 %0
}
