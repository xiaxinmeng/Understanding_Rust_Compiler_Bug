llvm
define void @_ZN10playground2f217h6945a283c9ef76d3E(i32* nocapture %a, i32* nocapture %b, i32* nocapture readonly %x) unnamed_addr #0 {
start:
  %0 = load i32, i32* %x, align 4
  store i32 %0, i32* %a, align 4
  %1 = load i32, i32* %x, align 4
  store i32 %1, i32* %b, align 4
  ret void
}
