
; ModuleID = 'test.test1.rcgu.no-opt.bc'
source_filename = "test1-8787f43e282added376259c1adb08b80.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::option::Option<Box<Foo<usize>>>" = type { [0 x i64], i64, [1 x i64] }
%"core::option::Option<Box<Foo<usize>>>::Some" = type { [1 x i64], i64*, [0 x i64] }

; Function Attrs: noinline uwtable
define void @_ZN4test7dealloc17hb146efc385cf7afbE(i64*) unnamed_addr #0 {
  ret void
}

; Function Attrs: uwtable
define i64 @_ZN4test3foo17h5eafaec26cbe1846E(%"core::option::Option<Box<Foo<usize>>>"* noalias nocapture dereferenceable(16)) unnamed_addr #1 {
  %2 = alloca i8, align 1
  %3 = alloca i8, align 1
  %4 = alloca i64, align 8
  store i8 0, i8* %3, align 1
  store i8 0, i8* %2, align 1
  store i8 1, i8* %2, align 1
  %5 = bitcast %"core::option::Option<Box<Foo<usize>>>"* %0 to i64*
  %6 = load i64, i64* %5, align 8, !range !0
  %7 = bitcast %"core::option::Option<Box<Foo<usize>>>"* %0 to i64*
  %8 = load i64, i64* %7, align 8, !range !0
  switch i64 %8, label %10 [
    i64 0, label %9
    i64 1, label %11
  ]

; <label>:9:                                      ; preds = %1
  store i64 0, i64* %4, align 8
  br label %21

; <label>:10:                                     ; preds = %1
  unreachable

; <label>:11:                                     ; preds = %1
  store i8 0, i8* %2, align 1
  store i8 1, i8* %3, align 1
  %12 = bitcast %"core::option::Option<Box<Foo<usize>>>"* %0 to %"core::option::Option<Box<Foo<usize>>>::Some"*
  %13 = getelementptr inbounds %"core::option::Option<Box<Foo<usize>>>::Some", %"core::option::Option<Box<Foo<usize>>>::Some"* %12, i32 0, i32 1
  %14 = load i64*, i64** %13, align 8
  %15 = load i64, i64* %14, align 8
  store i64 %15, i64* %4, align 8
  br label %21

; <label>:16:                                     ; preds = %21, %20
  store i8 0, i8* %3, align 1
  %17 = load i64, i64* %4, align 8
  %18 = bitcast %"core::option::Option<Box<Foo<usize>>>"* %0 to i64*
  %19 = load i64, i64* %18, align 8, !range !0
  switch i64 %19, label %31 [
    i64 1, label %25
  ]

; <label>:20:                                     ; preds = %21
  store i8 0, i8* %3, align 1
  call void @_ZN4test7dealloc17hb146efc385cf7afbE(i64* %14)
  br label %16

; <label>:21:                                     ; preds = %11, %9
  %22 = load i8, i8* %3, align 1, !range !1
  %23 = trunc i8 %22 to i1
  br i1 %23, label %20, label %16

; <label>:24:                                     ; preds = %31, %28, %25
  ret i64 %17

; <label>:25:                                     ; preds = %16
  %26 = load i8, i8* %2, align 1, !range !1
  %27 = trunc i8 %26 to i1
  br i1 %27, label %28, label %24

; <label>:28:                                     ; preds = %25
  store i8 0, i8* %2, align 1
  %29 = bitcast %"core::option::Option<Box<Foo<usize>>>"* %0 to %"core::option::Option<Box<Foo<usize>>>::Some"*
  %30 = getelementptr inbounds %"core::option::Option<Box<Foo<usize>>>::Some", %"core::option::Option<Box<Foo<usize>>>::Some"* %29, i32 0, i32 1
  call void @_ZN4core3ptr13drop_in_place17hc149ec1a0960208eE(i64** %30)
  br label %24

; <label>:31:                                     ; preds = %16
  call void @_ZN4core3ptr13drop_in_place17h2fef1d2b45619d3dE(%"core::option::Option<Box<Foo<usize>>>"* %0)
  br label %24
}

declare void @_ZN4core3ptr13drop_in_place17h2fef1d2b45619d3dE(%"core::option::Option<Box<Foo<usize>>>"*) unnamed_addr #2

declare void @_ZN4core3ptr13drop_in_place17hc149ec1a0960208eE(i64**) unnamed_addr #2

attributes #0 = { noinline uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { uwtable "probe-stack"="__rust_probestack" }
attributes #2 = { "probe-stack"="__rust_probestack" }

!0 = !{i64 0, i64 2}
!1 = !{i8 0, i8 2}
