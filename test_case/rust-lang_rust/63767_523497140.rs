asm
; ModuleID = 'integer_cmp.3a1fbbbh-cgu.0'
source_filename = "integer_cmp.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; core::cmp::impls::<impl core::cmp::Ord for i64>::cmp
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$i64$GT$3cmp17h54b2deb40875460aE"(i64* noalias readonly align 8 dereferenceable(8) %self, i64* noalias readonly align 8 dereferenceable(8) %other) unnamed_addr #0 {
start:
  %_0 = alloca i8, align 1
  %0 = load i64, i64* %self, align 8
  %1 = load i64, i64* %other, align 8
  %2 = icmp slt i64 %0, %1
  br i1 %2, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %3 = load i64, i64* %self, align 8
  %4 = load i64, i64* %other, align 8
  %5 = icmp sgt i64 %3, %4
  br i1 %5, label %bb4, label %bb3

bb2:                                              ; preds = %start
  store i8 -1, i8* %_0, align 1
  br label %bb6

bb3:                                              ; preds = %bb1
  store i8 0, i8* %_0, align 1
  br label %bb5

bb4:                                              ; preds = %bb1
  store i8 1, i8* %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6

bb6:                                              ; preds = %bb5, %bb2
  %6 = load i8, i8* %_0, align 1, !range !1
  ret i8 %6
}

; core::cmp::impls::<impl core::cmp::Ord for u32>::cmp
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$u32$GT$3cmp17h1dd39efa68a3677aE"(i32* noalias readonly align 4 dereferenceable(4) %self, i32* noalias readonly align 4 dereferenceable(4) %other) unnamed_addr #0 {
start:
  %_0 = alloca i8, align 1
  %0 = load i32, i32* %self, align 4
  %1 = load i32, i32* %other, align 4
  %2 = icmp ult i32 %0, %1
  br i1 %2, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %3 = load i32, i32* %self, align 4
  %4 = load i32, i32* %other, align 4
  %5 = icmp ugt i32 %3, %4
  br i1 %5, label %bb4, label %bb3

bb2:                                              ; preds = %start
  store i8 -1, i8* %_0, align 1
  br label %bb6

bb3:                                              ; preds = %bb1
  store i8 0, i8* %_0, align 1
  br label %bb5

bb4:                                              ; preds = %bb1
  store i8 1, i8* %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6

bb6:                                              ; preds = %bb5, %bb2
  %6 = load i8, i8* %_0, align 1, !range !1
  ret i8 %6
}

; Function Attrs: nonlazybind uwtable
define i8 @cmp_signed(i64, i64) unnamed_addr #1 {
start:
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  store i64 %0, i64* %a, align 8
  store i64 %1, i64* %b, align 8
; call core::cmp::impls::<impl core::cmp::Ord for i64>::cmp
  %2 = call i8 @"_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$i64$GT$3cmp17h54b2deb40875460aE"(i64* noalias readonly align 8 dereferenceable(8) %a, i64* noalias readonly align 8 dereferenceable(8) %b), !range !1
  br label %bb1

bb1:                                              ; preds = %start
  ret i8 %2
}

; Function Attrs: nonlazybind uwtable
define i8 @cmp_unsigned(i32, i32) unnamed_addr #1 {
start:
  %b = alloca i32, align 4
  %a = alloca i32, align 4
  store i32 %0, i32* %a, align 4
  store i32 %1, i32* %b, align 4
; call core::cmp::impls::<impl core::cmp::Ord for u32>::cmp
  %2 = call i8 @"_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$u32$GT$3cmp17h1dd39efa68a3677aE"(i32* noalias readonly align 4 dereferenceable(4) %a, i32* noalias readonly align 4 dereferenceable(4) %b), !range !1
  br label %bb1

bb1:                                              ; preds = %start
  ret i8 %2
}

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
!1 = !{i8 -1, i8 2}
