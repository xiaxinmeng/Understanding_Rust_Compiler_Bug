LLVM
define i1 @myfunction(i8 %0) {
start:
  %1 = icmp eq i8 %0, 47
  ret i1 %1
}
