
; ModuleID = '<stdin>'
source_filename = "primitive_types.arilfrs7-cgu.0"
target datalayout = "E-m:e-i1:8:16-i8:8:16-i64:64-f128:64-a:8:16-n32:64"
target triple = "s390x-unknown-linux-gnu"

define void @_ZN15primitive_types4U51211checked_sub17h5ff0d8bcee5c2f13E() unnamed_addr #0 {
start:
  %_5.sroa.4.0.copyload = load i64, i64* undef, align 8
  %0 = tail call { i64, i1 } @llvm.usub.with.overflow.i64(i64 undef, i64 %_5.sroa.4.0.copyload) #2
  %1 = extractvalue { i64, i1 } %0, 1
  %2 = zext i1 %1 to i8
  br i1 undef, label %bb21.i, label %bb34.i

bb21.i:                                           ; preds = %start
  %3 = tail call { i64, i1 } @llvm.usub.with.overflow.i64(i64 undef, i64 1) #2
  %4 = extractvalue { i64, i1 } %3, 1
  %5 = zext i1 %4 to i8
  %6 = add nuw nsw i8 %5, %2
  br label %bb34.i

bb34.i:                                           ; preds = %bb21.i, %start
  %carry.1.in.i = phi i8 [ %6, %bb21.i ], [ %2, %start ]
  ret void
}

; Function Attrs: nounwind readnone speculatable
declare { i64, i1 } @llvm.usub.with.overflow.i64(i64, i64) #1

attributes #0 = { "target-features"="-vector" }
attributes #1 = { nounwind readnone speculatable }
attributes #2 = { nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
