LLVM
define i1 @myfunction(i16 %0) {
start:
  %z = trunc i16 %0 to i8
  %1 = icmp ult i8 %z, 50
  %z.mask = and i16 %0, 255
  %2 = icmp eq i16 %z.mask, 47
  %3 = and i1 %1, %2
  ret i1 %3
}
