llvm
define float @example::calc(float) unnamed_addr #2 !dbg !22 {
  %1 = fmul float %0, 0x3FF3BE76C0000000, !dbg !24
  %2 = fdiv float %1, 5.000000e+00, !dbg !25
  ret float %2, !dbg !31
}
