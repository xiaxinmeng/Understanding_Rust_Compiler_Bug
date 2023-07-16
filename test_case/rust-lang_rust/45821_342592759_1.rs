llvm
define i8 @two_valued(i8) unnamed_addr #0 {
start:
  %not.cond = icmp ne i8 %0, 0
  %. = zext i1 %not.cond to i8
  ret i8 %.
}
