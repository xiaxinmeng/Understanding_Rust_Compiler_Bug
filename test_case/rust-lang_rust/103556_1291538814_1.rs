llvm
define noundef zeroext i1 @_ZN7example8demo_std17hcce6db1e74f1c1d4E(ptr noalias nocapture noundef readonly align 4 dereferenceable(4) %0, ptr noalias nocapture noundef readonly align 4 dereferenceable(4) %1) unnamed_addr #0 {
  %_9 = load i32, ptr %0, align 4
  %_10 = load i32, ptr %1, align 4
  %2 = icmp eq i32 %_9, %_10
  ret i1 %2
}
