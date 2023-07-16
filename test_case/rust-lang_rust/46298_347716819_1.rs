llvm
define zeroext i1 @foo(double %a) unnamed_addr #0 {
start:
  %0 = fptoui double %a to i64
  %1 = fcmp ogt double %a, 0x43EFFFFFFFFFFFFF
  %2 = icmp ne i64 %0, 0
  %not. = fcmp oge double %a, 0.000000e+00
  %3 = and i1 %not., %2
  %4 = or i1 %1, %3
  ret i1 %4
}
