llvm
define zeroext i1 @foo(double %a) unnamed_addr #0 {
start:
  %0 = fptoui double %a to i64
  %1 = icmp ne i64 %0, 0
  ret i1 %1
}
