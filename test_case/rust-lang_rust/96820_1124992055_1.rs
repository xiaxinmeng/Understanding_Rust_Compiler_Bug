llvm-ir
; ModuleID = 'example.6cc7c99e-cgu.0'
source_filename = "example.6cc7c99e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn
define void @foo({ i64, i32 }* nocapture noundef align 8 dereferenceable(16) %rc, i64* nocapture noundef align 8 dereferenceable(8) %r.0, i32* noalias nocapture noundef readonly align 4 dereferenceable(4) %r.1) unnamed_addr #0 {
start:
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn
define void @bar({ i64, i32 }* nocapture noundef align 8 dereferenceable(16) %rc, i64* nocapture noundef align 8 dereferenceable(16) %r) unnamed_addr #0 {
start:
  ret void
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
