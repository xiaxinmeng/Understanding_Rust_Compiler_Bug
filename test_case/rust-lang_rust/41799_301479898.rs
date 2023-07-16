llvm
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define internal i128 @foo() unnamed_addr {
start:
  ret i128 -1
}

define float @bar() unnamed_addr {
start:
  %0 = call i128 @foo()
  %1 = uitofp i128 %0 to float
  ret float %1
}
