llvm
target triple = "thumbv6m-none-unknown-eabi"

define i64 @func(i64 %arg) {
start:
  %0 = add i64 %arg, 1
  %1 = icmp ult i64 %0, 1           ; only `ult` can be used here fwr
  %2 = select i1 %1, i64 8, i64 0   ; 8 must be a power of 2
  ret i64 %2
}
