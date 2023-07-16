llvm
; ModuleID = 'playground.a04b7f1e-cgu.0'
source_filename = "playground.a04b7f1e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn
define { i32, i32 } @id_result(i32 noundef %0, i32 %1) unnamed_addr #0 {
start:
  %switch = icmp ne i32 %0, 0
  %. = zext i1 %switch to i32
  %2 = insertvalue { i32, i32 } undef, i32 %., 0
  %3 = insertvalue { i32, i32 } %2, i32 %1, 1
  ret { i32, i32 } %3
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
