llvm
; ModuleID = '<stdin>'
source_filename = "test0-6309762d5a7d9b368daa0ebe527434c3.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: norecurse readnone uwtable
define internal noalias nonnull align 8 dereferenceable(8) void (i32)** @0(i8** noalias readonly dereferenceable(8)) unnamed_addr #0 {
  %2 = bitcast i8** %0 to void (i32)**
  ret void (i32)** %2
}

; Function Attrs: norecurse readonly uwtable
define nonnull void (i32)* @"_ZN4test4main28_$u7b$$u7b$closure$u7d$$u7d$17hc0fea9cc3707b635E"(i8*) unnamed_addr #1 {
  %2 = alloca i8*, align 8
  %3 = call noalias align 8 dereferenceable(8) void (i32)** @0(i8** noalias readonly dereferenceable(8) %2)
  %4 = load void (i32)*, void (i32)** %3, !nonnull !1
  ret void (i32)* %4
}

attributes #0 = { norecurse readnone uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { norecurse readonly uwtable "probe-stack"="__rust_probestack" }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIE Level", i32 2}
!1 = !{}
