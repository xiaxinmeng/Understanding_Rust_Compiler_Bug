llvm
efine noundef zeroext i1 @_ZN7example12demo_obvious17haee70b6eb73f133dE(ptr noalias nocapture noundef readonly align 4 dereferenceable(4) %x, ptr noalias nocapture noundef readonly align 4 dereferenceable(4) %y) unnamed_addr #0 {
  %self = load i32, ptr %x, align 4, !range !2, !noundef !3
  %self1 = load i32, ptr %y, align 4, !range !2, !noundef !3
  %0 = icmp eq i32 %self, %self1
  ret i1 %0
}

!2 = !{i32 1, i32 0}
