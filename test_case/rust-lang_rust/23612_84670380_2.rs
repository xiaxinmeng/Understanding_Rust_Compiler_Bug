 llvm
define internal zeroext i1 @_ZN4main11closure.178E(%closure* noalias readonly, i32, i32) unnamed_addr #3 {
entry-block:
  %x = alloca i32
  %y = alloca i32
  store i32 %1, i32* %x
  store i32 %2, i32* %y
  %3 = load i32* %x
  %4 = load i32* %y
  %5 = icmp eq i32 %3, %4
  %6 = zext i1 %5 to i8
  %7 = trunc i8 %6 to i1
  ret i1 %7
}
