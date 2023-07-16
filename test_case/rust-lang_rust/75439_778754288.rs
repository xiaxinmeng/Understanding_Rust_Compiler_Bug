
issue75439 $ rustc +stage1_2 test.rs --crate-type=rlib --emit=llvm-ir -O
issue75439 $ cat test.ll
; ModuleID = 'test.3a1fbbbh-cgu.0'
source_filename = "test.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; test::foo3
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define i40 @_ZN4test4foo317h93f9bb6eba5df219E(i128 %0) unnamed_addr #0 {
start:
  %1 = and i128 %0, 18446744073709551615
  %2 = icmp eq i128 %1, 0
  br i1 %2, label %bb3, label %bb9

bb3:                                              ; preds = %start
  %_3.sroa.5.0.extract.shift = lshr i128 %0, 64
  %_3.sroa.5.0.extract.trunc = trunc i128 %_3.sroa.5.0.extract.shift to i32
  switch i32 %_3.sroa.5.0.extract.trunc, label %bb9 [
    i32 0, label %bb6
    i32 -65536, label %bb7
  ]

bb6:                                              ; preds = %bb3
  br label %bb9

bb7:                                              ; preds = %bb3
  br label %bb9

bb9:                                              ; preds = %bb7, %bb6, %start, %bb3
  %.sroa.0.0 = phi i40 [ 0, %bb3 ], [ 0, %start ], [ 1, %bb6 ], [ 1, %bb7 ]
  %3 = lshr i128 %0, 88
  %4 = trunc i128 %3 to i40
  %.sroa.3.0.insert.shift = and i40 %4, -256
  %.sroa.0.0.insert.insert = or i40 %.sroa.0.0, %.sroa.3.0.insert.shift
  ret i40 %.sroa.0.0.insert.insert
}

attributes #0 = { norecurse nounwind nonlazybind readnone uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
