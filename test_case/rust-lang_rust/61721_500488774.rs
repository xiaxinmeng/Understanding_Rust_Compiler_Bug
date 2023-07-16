
; ModuleID = 'reduced.ll'
source_filename = "memchr.53dfefha-cgu.0"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-none"

define internal i32 @0(<2 x i64>*) unnamed_addr #0 {
  %2 = call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> undef)
  ret i32 %2
}

; Function Attrs: nounwind readnone
declare i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8>) unnamed_addr #1

attributes #0 = { "target-features"="-mmx,-sse,+soft-float,+sse2" }
attributes #1 = { nounwind readnone }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
