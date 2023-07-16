
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "b.7rcbfp3g-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnux32"

define dso_local void @_ZN1b7call_it17h422679d04fa7b4d4E() unnamed_addr #0 {
start:
  %unsized_tmp = alloca i8, i32 undef, align 16
  unreachable
}

attributes #0 = { "probe-stack"="__rust_probestack" }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIE Level", i32 2}
