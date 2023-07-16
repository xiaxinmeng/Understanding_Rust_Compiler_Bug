llvm
; ModuleID = 'rust_out.b1180397-cgu.0'
source_filename = "rust_out.b1180397-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; Function Attrs: nonlazybind uwtable
define void @test() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %condition = tail call noundef zeroext i1 @random_bool()
  %num = tail call i64 @random_usize()
  %.not17 = icmp ne i64 %num, 0
  %0 = and i1 %condition, %.not17
  br i1 %0, label %bb9, label %bb11

bb11:                                             ; preds = %bb9, %start
  ret void

bb9:                                              ; preds = %start, %bb9
  %iter.sroa.0.016 = phi i64 [ %1, %bb9 ], [ 0, %start ]
  %1 = add nuw i64 %iter.sroa.0.016, 1
  tail call void @output(i64 %iter.sroa.0.016)
  %2 = icmp ult i64 %1, %num
  br i1 %2, label %bb9, label %bb11
}

; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @random_bool() unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare i64 @random_usize() unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @output(i64) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
