llvm
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define void @foo(<1 x i64>* noalias nocapture sret dereferenceable(8) %0, <1 x i64>* noalias nocapture readonly dereferenceable(8) %x) unnamed_addr #0 {
  %1 = getelementptr inbounds <1 x i64>, <1 x i64>* %x, i64 0, i64 0
  %2 = load i64, i64* %1, align 8
  %3 = getelementptr inbounds <1 x i64>, <1 x i64>* %0, i64 0, i64 0
  store i64 %2, i64* %3, align 8
  ret void
}

attributes #0 = { nofree norecurse nounwind nonlazybind uwtable "probe-stack"="__rust_probestack""target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
