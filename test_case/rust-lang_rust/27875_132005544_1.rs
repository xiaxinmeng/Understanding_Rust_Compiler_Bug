 llvm
define float @_ZN3foo20hc71bdc578cf276aaeaaE(float, float) unnamed_addr #0 {
entry-block:
  %dropflag_hint_6 = alloca i8
  %dropflag_hint_9 = alloca i8
  %a = alloca float
  %b = alloca float
  store i8 61, i8* %dropflag_hint_6
  store i8 61, i8* %dropflag_hint_9
  store float %0, float* %a, align 4
  store float %1, float* %b, align 4
  %2 = load float, float* %a, align 4
  %3 = load float, float* %b, align 4
  %4 = frem float %2, %3
  ret float %4
}
