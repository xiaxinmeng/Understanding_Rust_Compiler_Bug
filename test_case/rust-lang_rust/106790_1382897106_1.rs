llvm
; ModuleID = 'issue_103840.f9de1d84-cgu.0'
source_filename = "issue_103840.f9de1d84-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; issue_103840::foo
; Function Attrs: nonlazybind uwtable
define void @_ZN12issue_1038403foo17he2821c63cd4c4830E(ptr noalias nocapture noundef align 8 dereferenceable(24) %t) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %taken.sroa.0 = alloca { ptr, i64 }, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %taken.sroa.0)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %taken.sroa.0, ptr noundef nonnull align 8 dereferenceable(16) %t, i64 16, i1 false), !alias.scope !7, !noalias !5
  %taken.sroa.4.0.t.sroa_idx = getelementptr inbounds i8, ptr %t, i64 16
  %taken.sroa.4.0.copyload3 = load i64, ptr %taken.sroa.4.0.t.sroa_idx, align 8, !alias.scope !7, !noalias !5
  store ptr inttoptr (i64 8 to ptr), ptr %t, align 8, !alias.scope !9, !noalias !2
  %_9.sroa.4.0.t.sroa_idx = getelementptr inbounds i8, ptr %t, i64 8
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_9.sroa.4.0.t.sroa_idx, i8 0, i64 16, i1 false)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10)
  %_4.i.i.i.i = load i64, ptr %_9.sroa.4.0.t.sroa_idx, align 8, !alias.scope !13, !noalias !16
  %_3.i.i.i.i = icmp eq i64 %_4.i.i.i.i, 0
  br i1 %_3.i.i.i.i, label %bb4, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %start
  %_6.i.i.i.i.i = icmp ult i64 %_4.i.i.i.i, 1152921504606846976
  %array_size.i.i.i.i.i = shl nuw nsw i64 %_4.i.i.i.i, 3
  tail call void @llvm.assume(i1 %_6.i.i.i.i.i)
  tail call void @__rust_dealloc(ptr nonnull inttoptr (i64 8 to ptr), i64 %array_size.i.i.i.i.i, i64 8) #7, !noalias !10
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i, %start
  %0 = icmp eq i64 %taken.sroa.4.0.copyload3, 0
  %1 = add i64 %taken.sroa.4.0.copyload3, -1
  %spec.select = select i1 %0, i64 0, i64 %1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %t, ptr noundef nonnull align 8 dereferenceable(16) %taken.sroa.0, i64 16, i1 false)
  store i64 %spec.select, ptr %taken.sroa.4.0.t.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %taken.sroa.0)
  ret void
}

; Function Attrs: nonlazybind uwtable
declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, ptr, ptr) unnamed_addr #0

; Function Attrs: argmemonly mustprogress nocallback nofree nounwind willreturn
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #1

; Function Attrs: inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #2

; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #3

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #4

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #4

; Function Attrs: argmemonly nocallback nofree nounwind willreturn writeonly
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #5

; Function Attrs: inaccessiblememonly nocallback nofree nosync nounwind willreturn
declare void @llvm.experimental.noalias.scope.decl(metadata) #6

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { argmemonly mustprogress nocallback nofree nounwind willreturn }
attributes #2 = { inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn }
attributes #3 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { argmemonly mustprogress nocallback nofree nosync nounwind willreturn }
attributes #5 = { argmemonly nocallback nofree nounwind willreturn writeonly }
attributes #6 = { inaccessiblememonly nocallback nofree nosync nounwind willreturn }
attributes #7 = { nounwind }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN4core3mem7replace17h94c060526cb48374E: %result"}
!4 = distinct !{!4, !"_ZN4core3mem7replace17h94c060526cb48374E"}
!5 = !{!6}
!6 = distinct !{!6, !4, !"_ZN4core3mem7replace17h94c060526cb48374E: %src"}
!7 = !{!3, !8}
!8 = distinct !{!8, !4, !"_ZN4core3mem7replace17h94c060526cb48374E: %dest"}
!9 = !{!8, !6}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he64f84a0747c6173E: %self"}
!12 = distinct !{!12, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he64f84a0747c6173E"}
!13 = !{!14, !11}
!14 = distinct !{!14, !15, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hcd327976cb9281f7E: %self"}
!15 = distinct !{!15, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hcd327976cb9281f7E"}
!16 = !{!17}
!17 = distinct !{!17, !15, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hcd327976cb9281f7E: argument 0"}
