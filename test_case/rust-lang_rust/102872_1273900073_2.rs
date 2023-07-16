LLVM
define i1 @myfunction(i16 %0) {
start:
  %z = trunc i16 %0 to i8
  %1 = icmp ule i8 %z, 49
  %2 = zext i8 %z to i64
  %3 = add i64 %2, -46
  %_2 = select i1 %1, i64 %3, i64 0
  %4 = icmp eq i64 %_2, 1
  ret i1 %4
}
