llvm
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "4c72x1t0y1uzaw4e"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv6m-none-unknown-eabi"

define void @"_ZN50_$LT$u128$u20$as$u20$fixed..from_str..DecToBin$GT$10dec_to_bin17he7682fbe28887446E"() unnamed_addr #0 {
start:
  %0 = tail call { i128, i1 } @llvm.uadd.with.overflow.i128(i128 undef, i128 undef) #2
  %1 = extractvalue { i128, i1 } %0, 1
  %..i = select i1 %1, i128 18446744073709551616, i128 0
  %2 = tail call { i128, i1 } @llvm.uadd.with.overflow.i128(i128 undef, i128 %..i) #2
  %_70.1.i = extractvalue { i128, i1 } %2, 1
  br i1 %_70.1.i, label %panic3.i, label %_ZN5fixed8from_str9mul_hi_lo17h1c32783b5808c8e6E.exit

panic3.i:                                         ; preds = %start
  unreachable

_ZN5fixed8from_str9mul_hi_lo17h1c32783b5808c8e6E.exit: ; preds = %start
  unreachable
}

; Function Attrs: nounwind readnone speculatable willreturn
declare { i128, i1 } @llvm.uadd.with.overflow.i128(i128, i128) #1

attributes #0 = { "target-features"="+strict-align" }
attributes #1 = { nounwind readnone speculatable willreturn }
attributes #2 = { nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
