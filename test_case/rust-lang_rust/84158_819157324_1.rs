sh
$: rustc --version                                                       
rustc 1.51.0 (2fd73fabe 2021-03-23)
$: rustc --edition=2018 --emit=llvm-ir -C opt-level=0 extern-c-example.rs
$: /bin/cat extern-c-example.ll
; ModuleID = 'extern_c_example.3a1fbbbh-cgu.0'
source_filename = "extern_c_example.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; extern_c_example::huhu
; Function Attrs: nounwind nonlazybind uwtable
define void @_ZN16extern_c_example4huhu17h914345848a8b0e88E() unnamed_addr #0 {
start:
  ret void
}

attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
