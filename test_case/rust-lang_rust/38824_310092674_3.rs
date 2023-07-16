 llvm
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

define i128 @internal(i128, i128) unnamed_addr #0 {
start:
  %a = mul i128 %0, %1
  ret i128 %a
}

define ptx_kernel void @foo(i128, i128, i128*) unnamed_addr #0 {
start:
  %a = call i128 @internal(i128 %0, i128 %1)
  store i128 %a, i128* %2

  ret void
}

attributes #0 = { norecurse nounwind readnone }
