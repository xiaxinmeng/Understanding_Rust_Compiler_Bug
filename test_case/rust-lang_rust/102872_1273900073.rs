LLVM
define i1 @myfunction(i8 %0) {
start:
  %1 = icmp ule i8 %0, 49
  %2 = zext i8 %0 to i64
  %3 = add i64 %2, -46
  %_2 = select i1 %1, i64 %3, i64 0
  %4 = icmp eq i64 %_2, 1
  ret i1 %4
}

