
// If this is x86-64, and we disabled SSE, we can't return FP values,
// or SSE or MMX vectors.
if ((ValVT == MVT::f32 || ValVT == MVT::f64 ||
     VA.getLocReg() == X86::XMM0 || VA.getLocReg() == X86::XMM1) &&
      (Subtarget->is64Bit() && !Subtarget->hasSSE1())) {
  report_fatal_error("SSE register return with SSE disabled");
}
