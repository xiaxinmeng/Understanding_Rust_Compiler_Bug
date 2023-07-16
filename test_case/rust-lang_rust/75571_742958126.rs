
; ModuleID = 'array_map.3a1fbbbh-cgu.0'
source_filename = "array_map.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>" = type { [0 x i8], %"[closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]", [0 x i8], %"std::array::IntoIter<u32, 4_usize>", [0 x i64] }
%"[closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]" = type {}
%"std::array::IntoIter<u32, 4_usize>" = type { [0 x i64], { i64, i64 }, [0 x i32], [4 x i32], [0 x i32] }
%"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>" = type { [0 x i8], %"[closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]", [0 x i8], %"std::array::IntoIter<u32, 4_usize>", [0 x i64] }
%"[closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]" = type {}
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; Function Attrs: nounwind nonlazybind uwtable
define i128 @array_cast_to_float(i128 %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %tmp.i.i.i.i.i2.i.i.i.i = alloca i32, align 4
  %tmp.i.i.i.i.i.i.i.i.i = alloca i64, align 8
  %src.i.i.i.i.i.i = alloca i64, align 8
  %iter1.i.i = alloca %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", align 16
  %tmp.i.i.i.i = alloca i128, align 8
  %array.i.i = alloca i128, align 8
  %1 = bitcast i128* %array.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %1)
  store i128 %0, i128* %array.i.i, align 8, !noalias !2
  %2 = bitcast i128* %tmp.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %2) #3, !noalias !5
  %_6.i.i.i.i.i.i = ptrtoint i128* %array.i.i to i64
  %_6.i2.i.i.i.i.i = ptrtoint i128* %tmp.i.i.i.i to i64
  %_13.i.i.i.i.i.i = icmp ult i128* %tmp.i.i.i.i, %array.i.i
  %3 = sub i64 %_6.i.i.i.i.i.i, %_6.i2.i.i.i.i.i
  %4 = sub i64 %_6.i2.i.i.i.i.i, %_6.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i, i64 %3, i64 %4
  %5 = icmp ugt i64 %diff.0.i.i.i.i.i.i, 15
  br i1 %5, label %bb3.i.i.i.lr.ph.i.i, label %bb16.i.i.i.i.i

bb16.i.i.i.i.i:                                   ; preds = %start
  call void @llvm.trap() #3, !noalias !2
  unreachable

bb3.i.i.i.lr.ph.i.i:                              ; preds = %start
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(16) %2, i8* nonnull align 8 dereferenceable(16) %1, i64 16, i1 false) #3, !noalias !2
  %_7.sroa.0.0.copyload.i.i.i.i = load i128, i128* %tmp.i.i.i.i, align 8, !noalias !5
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %2) #3, !noalias !5
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %1)
  %6 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i, i64 0, i32 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %6), !noalias !8
  %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i to i64*
  %7 = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i to <2 x i64>*
  store <2 x i64> <i64 0, i64 4>, <2 x i64>* %7, align 16
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i, i64 0, i32 3, i32 3
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast [4 x i32]* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i to i128*
  store i128 %_7.sroa.0.0.copyload.i.i.i.i, i128* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16
  %8 = bitcast i64* %src.i.i.i.i.i.i to i8*
  %_6.i.i.i.i.i.i.i.i.i.i.i = ptrtoint %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i to i64
  %_6.i2.i.i.i.i.i.i.i.i.i.i = ptrtoint i64* %tmp.i.i.i.i.i.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i.i.i.i.i = icmp ult i64* %tmp.i.i.i.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %9 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i2.i.i.i.i.i.i.i.i.i.i
  %10 = sub i64 %_6.i2.i.i.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i.i, i64 %9, i64 %10
  %.not.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i.i, 0
  %_6.i.i4.i.i.i.i.i.i.i.i = ptrtoint i64* %src.i.i.i.i.i.i to i64
  %11 = bitcast i32* %tmp.i.i.i.i.i2.i.i.i.i to i8*
  %_6.i2.i.i.i.i.i.i4.i.i.i.i = ptrtoint i32* %tmp.i.i.i.i.i2.i.i.i.i to i64
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.us.i.i, label %bb3.i.i.i.lr.ph.split.i.i

bb3.i.i.i.us.i.i:                                 ; preds = %bb3.i.i.i.lr.ph.i.i
  %12 = bitcast i64* %tmp.i.i.i.i.i.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %8) #3, !noalias !8
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !8
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %12) #3, !noalias !8
  call void @llvm.trap() #3, !noalias !8
  unreachable

bb3.i.i.i.lr.ph.split.i.i:                        ; preds = %bb3.i.i.i.lr.ph.i.i
  %_13.i.i.i.i.i.i.i.i.i.i = icmp ugt i64* %src.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %13 = sub i64 %_6.i.i4.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %14 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i.i4.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i, i64 %13, i64 %14
  %.not1.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not1.i.i.i.i.i.i.i, label %bb3.i.i.i.us40.i.i, label %bb3.i.i.i.i.preheader.i

bb3.i.i.i.i.preheader.i:                          ; preds = %bb3.i.i.i.lr.ph.split.i.i
  %15 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 0
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %_6.i.i.i.i.i.i.i3.i.i.i.i = ptrtoint i32* %15 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %15
  %16 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %17 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.i, i64 %16, i64 %17
  %.not.i.i.i7.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.i, 0
  br i1 %.not.i.i.i7.i.i.i.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.i"

bb3.i.i.i.us40.i.i:                               ; preds = %bb3.i.i.i.lr.ph.split.i.i
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %8) #3, !noalias !8
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !8
  call void @llvm.trap() #3, !noalias !8
  unreachable

bb16.i.i.i.i.i.i9.i.i.i.i:                        ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.2.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.1.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.i", %bb3.i.i.i.i.preheader.i
  %.lcssa.i = phi i64 [ 1, %bb3.i.i.i.i.preheader.i ], [ 2, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.i" ], [ 3, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.1.i" ], [ 4, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.2.i" ]
  store i64 %.lcssa.i, i64* %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !8
  call void @llvm.trap() #3, !noalias !18
  unreachable

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.i": ; preds = %bb3.i.i.i.i.preheader.i
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %18 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 1
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %_6.i.i.i.i.i.i.i3.i.i.i.1.i = ptrtoint i32* %18 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.1.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %18
  %19 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.1.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %20 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.1.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.1.i, i64 %19, i64 %20
  %.not.i.i.i7.i.i.i.1.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i, 0
  br i1 %.not.i.i.i7.i.i.i.1.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.1.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.1.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %21 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 2
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %_6.i.i.i.i.i.i.i3.i.i.i.2.i = ptrtoint i32* %21 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.2.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %21
  %22 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.2.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %23 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.2.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.2.i, i64 %22, i64 %23
  %.not.i.i.i7.i.i.i.2.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i, 0
  br i1 %.not.i.i.i7.i.i.i.2.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.2.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.2.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.1.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %24 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:14:9: 14:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 3
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %_6.i.i.i.i.i.i.i3.i.i.i.3.i = ptrtoint i32* %24 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.3.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %24
  %25 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.3.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %26 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.3.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.3.i, i64 %25, i64 %26
  %.not.i.i.i7.i.i.i.3.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i, 0
  br i1 %.not.i.i.i7.i.i.i.3.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17hdb07c6c58db3582cE.exit"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17hdb07c6c58db3582cE.exit": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h4ac8fd7710fd6f9dE.exit.i.2.i"
  %27 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 64
  %28 = trunc i128 %27 to i32
  %phi.cast.i.i.2.i = uitofp i32 %28 to float
  %29 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 32
  %30 = trunc i128 %29 to i32
  %phi.cast.i.i.1.i = uitofp i32 %30 to float
  %31 = trunc i128 %_7.sroa.0.0.copyload.i.i.i.i to i32
  %phi.cast.i.i.i = uitofp i32 %31 to float
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %11) #3, !noalias !11
  %32 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 96
  %33 = trunc i128 %32 to i32
  %phi.cast.i.i.3.i = uitofp i32 %33 to float
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %6), !noalias !8
  %34 = bitcast float %phi.cast.i.i.i to i32
  %35 = bitcast float %phi.cast.i.i.1.i to i32
  %36 = bitcast float %phi.cast.i.i.2.i to i32
  %37 = bitcast float %phi.cast.i.i.3.i to i32
  %buffer.i.sroa.6.0.insert.ext = zext i32 %37 to i128
  %buffer.i.sroa.6.0.insert.shift = shl nuw i128 %buffer.i.sroa.6.0.insert.ext, 96
  %buffer.i.sroa.5.0.insert.ext = zext i32 %36 to i128
  %buffer.i.sroa.5.0.insert.shift = shl nuw nsw i128 %buffer.i.sroa.5.0.insert.ext, 64
  %buffer.i.sroa.4.0.insert.ext = zext i32 %35 to i128
  %buffer.i.sroa.4.0.insert.shift = shl nuw nsw i128 %buffer.i.sroa.4.0.insert.ext, 32
  %buffer.i.sroa.0.0.insert.ext = zext i32 %34 to i128
  %buffer.i.sroa.5.0.insert.insert = or i128 %buffer.i.sroa.5.0.insert.shift, %buffer.i.sroa.0.0.insert.ext
  %buffer.i.sroa.4.0.insert.insert = or i128 %buffer.i.sroa.5.0.insert.insert, %buffer.i.sroa.6.0.insert.shift
  %buffer.i.sroa.0.0.insert.insert = or i128 %buffer.i.sroa.4.0.insert.insert, %buffer.i.sroa.4.0.insert.shift
  ret i128 %buffer.i.sroa.0.0.insert.insert
}

; Function Attrs: nounwind nonlazybind uwtable
define void @array_cast_to_u64([4 x i64]* noalias nocapture sret dereferenceable(32) %0, i128 %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %tmp.i.i.i.i.i2.i.i.i.i = alloca i32, align 4
  %tmp.i.i.i.i.i.i.i.i.i = alloca i64, align 8
  %src.i.i.i.i.i.i = alloca i64, align 8
  %iter1.i.i = alloca %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", align 16
  %tmp.i.i.i.i = alloca i128, align 8
  %array.i.i = alloca i128, align 8
  %2 = bitcast i128* %array.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %2), !noalias !19
  store i128 %1, i128* %array.i.i, align 8, !noalias !22
  %3 = bitcast i128* %tmp.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %3) #3, !noalias !25
  %_6.i.i.i.i.i.i = ptrtoint i128* %array.i.i to i64
  %_6.i2.i.i.i.i.i = ptrtoint i128* %tmp.i.i.i.i to i64
  %_13.i.i.i.i.i.i = icmp ult i128* %tmp.i.i.i.i, %array.i.i
  %4 = sub i64 %_6.i.i.i.i.i.i, %_6.i2.i.i.i.i.i
  %5 = sub i64 %_6.i2.i.i.i.i.i, %_6.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i, i64 %4, i64 %5
  %6 = icmp ugt i64 %diff.0.i.i.i.i.i.i, 15
  br i1 %6, label %bb3.i.i.i.lr.ph.i.i, label %bb16.i.i.i.i.i

bb16.i.i.i.i.i:                                   ; preds = %start
  call void @llvm.trap() #3, !noalias !22
  unreachable

bb3.i.i.i.lr.ph.i.i:                              ; preds = %start
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(16) %3, i8* nonnull align 8 dereferenceable(16) %2, i64 16, i1 false) #3, !noalias !22
  %_7.sroa.0.0.copyload.i.i.i.i = load i128, i128* %tmp.i.i.i.i, align 8, !noalias !25
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %3) #3, !noalias !25
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %2), !noalias !19
  %7 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i, i64 0, i32 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %7), !noalias !28
  %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i to i64*
  %8 = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i to <2 x i64>*
  store <2 x i64> <i64 0, i64 4>, <2 x i64>* %8, align 16, !noalias !19
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i, i64 0, i32 3, i32 3
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast [4 x i32]* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i to i128*
  store i128 %_7.sroa.0.0.copyload.i.i.i.i, i128* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !19
  %9 = bitcast i64* %src.i.i.i.i.i.i to i8*
  %_6.i.i.i.i.i.i.i.i.i.i.i = ptrtoint %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i to i64
  %_6.i2.i.i.i.i.i.i.i.i.i.i = ptrtoint i64* %tmp.i.i.i.i.i.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i.i.i.i.i = icmp ult i64* %tmp.i.i.i.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %10 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i2.i.i.i.i.i.i.i.i.i.i
  %11 = sub i64 %_6.i2.i.i.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i.i, i64 %10, i64 %11
  %.not.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i.i, 0
  %_6.i.i4.i.i.i.i.i.i.i.i = ptrtoint i64* %src.i.i.i.i.i.i to i64
  %12 = bitcast i32* %tmp.i.i.i.i.i2.i.i.i.i to i8*
  %_6.i2.i.i.i.i.i.i4.i.i.i.i = ptrtoint i32* %tmp.i.i.i.i.i2.i.i.i.i to i64
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.us.i.i, label %bb3.i.i.i.lr.ph.split.i.i

bb3.i.i.i.us.i.i:                                 ; preds = %bb3.i.i.i.lr.ph.i.i
  %13 = bitcast i64* %tmp.i.i.i.i.i.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %9) #3, !noalias !28
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !28
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %13) #3, !noalias !28
  call void @llvm.trap() #3, !noalias !28
  unreachable

bb3.i.i.i.lr.ph.split.i.i:                        ; preds = %bb3.i.i.i.lr.ph.i.i
  %_13.i.i.i.i.i.i.i.i.i.i = icmp ugt i64* %src.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %14 = sub i64 %_6.i.i4.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %15 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i.i4.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i, i64 %14, i64 %15
  %.not1.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not1.i.i.i.i.i.i.i, label %bb3.i.i.i.us40.i.i, label %bb3.i.i.i.i.preheader.i

bb3.i.i.i.i.preheader.i:                          ; preds = %bb3.i.i.i.lr.ph.split.i.i
  %16 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 0
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %_6.i.i.i.i.i.i.i3.i.i.i.i = ptrtoint i32* %16 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %16
  %17 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %18 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.i, i64 %17, i64 %18
  %.not.i.i.i7.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.i, 0
  br i1 %.not.i.i.i7.i.i.i.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.i"

bb3.i.i.i.us40.i.i:                               ; preds = %bb3.i.i.i.lr.ph.split.i.i
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %9) #3, !noalias !28
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !28
  call void @llvm.trap() #3, !noalias !28
  unreachable

bb16.i.i.i.i.i.i9.i.i.i.i:                        ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.2.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.1.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.i", %bb3.i.i.i.i.preheader.i
  %.lcssa.i = phi i64 [ 1, %bb3.i.i.i.i.preheader.i ], [ 2, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.i" ], [ 3, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.1.i" ], [ 4, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.2.i" ]
  store i64 %.lcssa.i, i64* %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !28
  call void @llvm.trap() #3, !noalias !38
  unreachable

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.i": ; preds = %bb3.i.i.i.i.preheader.i
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %19 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 1
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %_6.i.i.i.i.i.i.i3.i.i.i.1.i = ptrtoint i32* %19 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.1.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %19
  %20 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.1.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %21 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.1.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.1.i, i64 %20, i64 %21
  %.not.i.i.i7.i.i.i.1.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i, 0
  br i1 %.not.i.i.i7.i.i.i.1.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.1.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.1.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %22 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 2
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %_6.i.i.i.i.i.i.i3.i.i.i.2.i = ptrtoint i32* %22 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.2.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %22
  %23 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.2.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %24 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.2.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.2.i, i64 %23, i64 %24
  %.not.i.i.i7.i.i.i.2.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i, 0
  br i1 %.not.i.i.i7.i.i.i.2.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.2.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.2.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.1.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %25 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:25:9: 25:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 3
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %_6.i.i.i.i.i.i.i3.i.i.i.3.i = ptrtoint i32* %25 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.3.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %25
  %26 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.3.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %27 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.3.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.3.i, i64 %26, i64 %27
  %.not.i.i.i7.i.i.i.3.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i, 0
  br i1 %.not.i.i.i7.i.i.i.3.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h67a9fe77bbbd92e0E.exit"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h67a9fe77bbbd92e0E.exit": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h20a5840875d0cfddE.exit.i.2.i"
  %28 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 64
  %29 = trunc i128 %28 to i64
  %phi.cast.i.i.2.i = and i64 %29, 4294967295
  %30 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 32
  %31 = insertelement <2 x i128> undef, i128 %_7.sroa.0.0.copyload.i.i.i.i, i32 0
  %32 = insertelement <2 x i128> %31, i128 %30, i32 1
  %33 = trunc <2 x i128> %32 to <2 x i64>
  %34 = and <2 x i64> %33, <i64 4294967295, i64 4294967295>
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !31
  %35 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 96
  %36 = trunc i128 %35 to i64
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %7), !noalias !28
  %37 = bitcast [4 x i64]* %0 to <2 x i64>*
  store <2 x i64> %34, <2 x i64>* %37, align 8
  %buffer.i.sroa.5.0..sroa_idx7 = getelementptr inbounds [4 x i64], [4 x i64]* %0, i64 0, i64 2
  store i64 %phi.cast.i.i.2.i, i64* %buffer.i.sroa.5.0..sroa_idx7, align 8
  %buffer.i.sroa.6.0..sroa_idx9 = getelementptr inbounds [4 x i64], [4 x i64]* %0, i64 0, i64 3
  store i64 %36, i64* %buffer.i.sroa.6.0..sroa_idx9, align 8
  ret void
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #2

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0

attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { argmemonly nounwind willreturn }
attributes #2 = { cold noreturn nounwind }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17h08b5d275cb899f9cE: %iter"}
!4 = distinct !{!4, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17h08b5d275cb899f9cE"}
!5 = !{!6, !3}
!6 = distinct !{!6, !7, !"_ZN4core3mem14transmute_copy17hdca171785ff334f2E: %src"}
!7 = distinct !{!7, !"_ZN4core3mem14transmute_copy17hdca171785ff334f2E"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17h8248582cb705ef25E: %iter"}
!10 = distinct !{!10, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17h8248582cb705ef25E"}
!11 = !{!12, !14, !16, !9}
!12 = distinct !{!12, !13, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17hace13e682fd6db62E: %self"}
!13 = distinct !{!13, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17hace13e682fd6db62E"}
!14 = distinct !{!14, !15, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h5922dc1006c3a4a1E: %_1"}
!15 = distinct !{!15, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h5922dc1006c3a4a1E"}
!16 = distinct !{!16, !17, !"_ZN4core6option15Option$LT$T$GT$3map17hf8dbcf446d7e5ba8E: %f"}
!17 = distinct !{!17, !"_ZN4core6option15Option$LT$T$GT$3map17hf8dbcf446d7e5ba8E"}
!18 = !{!14, !16, !9}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h67a9fe77bbbd92e0E: argument 0"}
!21 = distinct !{!21, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h67a9fe77bbbd92e0E"}
!22 = !{!23, !20}
!23 = distinct !{!23, !24, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17h08b5d275cb899f9cE: %iter"}
!24 = distinct !{!24, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17h08b5d275cb899f9cE"}
!25 = !{!26, !23, !20}
!26 = distinct !{!26, !27, !"_ZN4core3mem14transmute_copy17hdca171785ff334f2E: %src"}
!27 = distinct !{!27, !"_ZN4core3mem14transmute_copy17hdca171785ff334f2E"}
!28 = !{!29, !20}
!29 = distinct !{!29, !30, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17hdd217a46e8be43e7E: %iter"}
!30 = distinct !{!30, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17hdd217a46e8be43e7E"}
!31 = !{!32, !34, !36, !29, !20}
!32 = distinct !{!32, !33, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17hace13e682fd6db62E: %self"}
!33 = distinct !{!33, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17hace13e682fd6db62E"}
!34 = distinct !{!34, !35, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h5922dc1006c3a4a1E: %_1"}
!35 = distinct !{!35, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h5922dc1006c3a4a1E"}
!36 = distinct !{!36, !37, !"_ZN4core6option15Option$LT$T$GT$3map17hf8dbcf446d7e5ba8E: %f"}
!37 = distinct !{!37, !"_ZN4core6option15Option$LT$T$GT$3map17hf8dbcf446d7e5ba8E"}
!38 = !{!34, !36, !29, !20}
