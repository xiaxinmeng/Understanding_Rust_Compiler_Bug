llvm
define void @_ZN10playground8sum_rust17hf4e374897bed05a9E(<4 x float>* noalias nocapture sret(<4 x float>) dereferenceable(16) %0, <4 x float>* noalias nocapture readonly align 16 dereferenceable(16) %a, <4 x float>* noalias nocapture readonly align 16 dereferenceable(16) %b) unnamed_addr #0 {
start:
  %1 = load <4 x float>, <4 x float>* %a, align 16
  %2 = load <4 x float>, <4 x float>* %b, align 16
  %3 = fadd <4 x float> %1, %2
  store <4 x float> %3, <4 x float>* %0, align 16
  ret void
}
