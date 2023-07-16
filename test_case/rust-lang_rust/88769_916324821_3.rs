llvm
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "issue_88769_min.61742fa9-cgu.3"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define dso_local void @_ZN15issue_88769_min4main17h9167823f857de443E() unnamed_addr #0 {
  %1 = call i8 @llvm.vector.reduce.mul.v16i8(<16 x i8> bitcast (<1 x i128> <i128 20011376718272490338853433276725592320> to <16 x i8>))
  store i8 %1, i8* undef, align 1
  ret void
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i8 @llvm.vector.reduce.mul.v16i8(<16 x i8>) #1

attributes #0 = { "target-cpu"="x86-64" }
attributes #1 = { nofree nosync nounwind readnone willreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
