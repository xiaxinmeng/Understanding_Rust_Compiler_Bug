llvm
; ModuleID = 'issue_103840.f9de1d84-cgu.0'
source_filename = "issue_103840.f9de1d84-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; issue_103840::foo
; Function Attrs: nonlazybind uwtable
define void @_ZN12issue_1038403foo17he2821c63cd4c4830E(ptr noalias nocapture noundef align 8 dereferenceable(24) %t) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %taken.sroa.0 = alloca { i64, ptr }, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %taken.sroa.0)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %taken.sroa.0, ptr noundef nonnull align 8 dereferenceable(16) %t, i64 16, i1 false), !alias.scope !2, !noalias !6
  %taken.sroa.4.0.t.sroa_idx = getelementptr inbounds i8, ptr %t, i64 16
  %taken.sroa.4.0.copyload3 = load i64, ptr %taken.sroa.4.0.t.sroa_idx, align 8, !alias.scope !2, !noalias !6
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

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #2

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #2

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { argmemonly mustprogress nocallback nofree nounwind willreturn }
attributes #2 = { argmemonly mustprogress nocallback nofree nosync nounwind willreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!3, !5}
!3 = distinct !{!3, !4, !"_ZN4core3mem7replace17h94c060526cb48374E: %result"}
!4 = distinct !{!4, !"_ZN4core3mem7replace17h94c060526cb48374E"}
!5 = distinct !{!5, !4, !"_ZN4core3mem7replace17h94c060526cb48374E: %dest"}
!6 = !{!7}
!7 = distinct !{!7, !4, !"_ZN4core3mem7replace17h94c060526cb48374E: %src"}

