llvm
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define void @_ZN7example3foo17h2ac4b13db8a0abecE
  (<2 x double>* noalias nocapture sret dereferenceable(16) %0, double %x, double %y)
  unnamed_addr #0 !dbg !6 {
    store <2 x double> <double 0.000000e+00, double 1.000000e+00>, <2 x double>* %0, align 16, !dbg !10
    ret void, !dbg !11
}

attributes #0 = { nofree norecurse nounwind nonlazybind uwtable writeonly
"probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
