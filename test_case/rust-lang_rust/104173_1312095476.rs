ll
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "repro.dba56e2e-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@_ZN5repro3FOO7__getit5__KEY17h8a371060d0f5b551E = external thread_local global <{ [4 x i8], [4 x i8], [1 x i8], [3 x i8] }>, align 4

declare ptr @"_ZN3std6thread5local4fast12Key$LT$T$GT$3get17h336c607071ef2f69E"(ptr, ptr) unnamed_addr #0

define hidden ptr @_ZN5repro3FOO7__getit17hcc9532ca2598edd1E(ptr %init) unnamed_addr #0 {
start:
  %0 = call align 4 ptr @"_ZN3std6thread5local4fast12Key$LT$T$GT$3get17h336c607071ef2f69E"(ptr align 4 @_ZN5repro3FOO7__getit5__KEY17h8a371060d0f5b551E, ptr align 4 undef)
  unreachable
}

attributes #0 = { "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
