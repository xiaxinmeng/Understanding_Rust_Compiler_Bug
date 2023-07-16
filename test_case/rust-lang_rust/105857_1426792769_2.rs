
; ModuleID = 'fibonacci_vec.ll'
source_filename = "fibonacci_vec.f1b28543-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

; Function Attrs: nonlazybind uwtable
define void @_ZN13fibonacci_vec13fibonacci_vec17hb6b1caf079e597beE(ptr noalias nocapture noundef nonnull align 8 %fib.0, i64 %fib.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ugt i64 %fib.1, 2
  br i1 %0, label %bb7.preheader, label %bb4

bb7.preheader:                                    ; preds = %start
  %uglygep = getelementptr i8, ptr %fib.0, i64 8
  %load_initial = load i64, ptr %uglygep, align 8
  br label %bb7

bb4:                                              ; preds = %bb7, %start
  ret void

bb7:                                              ; preds = %bb7.preheader, %bb7
  %store_forwarded = phi i64 [ %load_initial, %bb7.preheader ], [ %4, %bb7 ]
  %iter.sroa.0.05 = phi i64 [ 2, %bb7.preheader ], [ %1, %bb7 ]
  %1 = add nuw i64 %iter.sroa.0.05, 1
  %_18 = add i64 %iter.sroa.0.05, -2
  %2 = getelementptr inbounds [0 x i64], ptr %fib.0, i64 0, i64 %_18
  %_17 = load i64, ptr %2, align 8
  %3 = getelementptr inbounds [0 x i64], ptr %fib.0, i64 0, i64 %iter.sroa.0.05
  %4 = add i64 %_17, %store_forwarded
  store i64 %4, ptr %3, align 8
  %exitcond.not = icmp eq i64 %1, %fib.1
  br i1 %exitcond.not, label %bb4, label %bb7
}

; Function Attrs: nonlazybind uwtable
declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, ptr, ptr) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics,+v8a" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
