
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; Function Attrs: nounwind nonlazybind uwtable
define i128 @array_cast_to_float([4 x i32]* noalias nocapture readonly dereferenceable(16) %x) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %tmp.i.i.i.i.i2.i.i.i.i = alloca i32, align 4
  %tmp.i.i.i.i.i.i.i.i.i = alloca i64, align 8
  %src.i.i.i.i.i.i = alloca i64, align 8
  %iter1.i.i = alloca %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", align 16
  %tmp.i.i.i.i = alloca i128, align 8
  %_10.i = alloca [4 x i32], align 4
  %0 = bitcast [4 x i32]* %x to i8*
  %1 = bitcast [4 x i32]* %_10.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %1), !noalias !2
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 4 dereferenceable(16) %1, i8* nonnull align 4 dereferenceable(16) %0, i64 16, i1 false)
  %2 = bitcast i128* %tmp.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %2) #3, !noalias !5
  %_6.i.i.i.i.i.i = ptrtoint [4 x i32]* %_10.i to i64
  %3 = bitcast i128* %tmp.i.i.i.i to [4 x i32]*
  %_6.i2.i.i.i.i.i = ptrtoint i128* %tmp.i.i.i.i to i64
  %_13.i.i.i.i.i.i = icmp ugt [4 x i32]* %_10.i, %3
  %4 = sub i64 %_6.i.i.i.i.i.i, %_6.i2.i.i.i.i.i
  %5 = sub i64 %_6.i2.i.i.i.i.i, %_6.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i, i64 %4, i64 %5
  %6 = icmp ugt i64 %diff.0.i.i.i.i.i.i, 15
  br i1 %6, label %bb3.i.i.i.lr.ph.i.i, label %bb16.i.i.i.i.i

bb16.i.i.i.i.i:                                   ; preds = %start
  call void @llvm.trap() #3, !noalias !11
  unreachable

bb3.i.i.i.lr.ph.i.i:                              ; preds = %start
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(16) %2, i8* nonnull align 4 dereferenceable(16) %1, i64 16, i1 false) #3, !noalias !11
  %_7.sroa.0.0.copyload.i.i.i.i = load i128, i128* %tmp.i.i.i.i, align 8, !noalias !5
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %2) #3, !noalias !5
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %1), !noalias !2
  %7 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i, i64 0, i32 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %7), !noalias !12
  %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i to i64*
  %8 = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i to <2 x i64>*
  store <2 x i64> <i64 0, i64 4>, <2 x i64>* %8, align 16, !noalias !2
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i, i64 0, i32 3, i32 3
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast [4 x i32]* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i to i128*
  store i128 %_7.sroa.0.0.copyload.i.i.i.i, i128* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !2
  %9 = bitcast i64* %src.i.i.i.i.i.i to i8*
  %_6.i.i.i.i.i.i.i.i.i.i.i = ptrtoint %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i to i64
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
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %9) #3, !noalias !12
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !12
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %13) #3, !noalias !12
  call void @llvm.trap() #3, !noalias !12
  unreachable

bb3.i.i.i.lr.ph.split.i.i:                        ; preds = %bb3.i.i.i.lr.ph.i.i
  %_13.i.i.i.i.i.i.i.i.i.i = icmp ugt i64* %src.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %14 = sub i64 %_6.i.i4.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %15 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i.i4.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i, i64 %14, i64 %15
  %.not1.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not1.i.i.i.i.i.i.i, label %bb3.i.i.i.us40.i.i, label %bb3.i.i.i.i.preheader.i

bb3.i.i.i.i.preheader.i:                          ; preds = %bb3.i.i.i.lr.ph.split.i.i
  %16 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 0
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %_6.i.i.i.i.i.i.i3.i.i.i.i = ptrtoint i32* %16 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %16
  %17 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %18 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.i, i64 %17, i64 %18
  %.not.i.i.i7.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.i, 0
  br i1 %.not.i.i.i7.i.i.i.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.i"

bb3.i.i.i.us40.i.i:                               ; preds = %bb3.i.i.i.lr.ph.split.i.i
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %9) #3, !noalias !12
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !12
  call void @llvm.trap() #3, !noalias !12
  unreachable

bb16.i.i.i.i.i.i9.i.i.i.i:                        ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.2.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.1.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.i", %bb3.i.i.i.i.preheader.i
  %.lcssa.i = phi i64 [ 1, %bb3.i.i.i.i.preheader.i ], [ 2, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.i" ], [ 3, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.1.i" ], [ 4, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.2.i" ]
  store i64 %.lcssa.i, i64* %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !12
  call void @llvm.trap() #3, !noalias !22
  unreachable

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.i": ; preds = %bb3.i.i.i.i.preheader.i
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %19 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 1
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %_6.i.i.i.i.i.i.i3.i.i.i.1.i = ptrtoint i32* %19 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.1.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %19
  %20 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.1.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %21 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.1.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.1.i, i64 %20, i64 %21
  %.not.i.i.i7.i.i.i.1.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i, 0
  br i1 %.not.i.i.i7.i.i.i.1.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.1.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.1.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %22 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 2
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %_6.i.i.i.i.i.i.i3.i.i.i.2.i = ptrtoint i32* %22 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.2.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %22
  %23 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.2.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %24 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.2.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.2.i, i64 %23, i64 %24
  %.not.i.i.i7.i.i.i.2.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i, 0
  br i1 %.not.i.i.i7.i.i.i.2.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.2.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.2.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.1.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %25 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:12:9: 12:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 3
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %_6.i.i.i.i.i.i.i3.i.i.i.3.i = ptrtoint i32* %25 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.3.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %25
  %26 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.3.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %27 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.3.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.3.i, i64 %26, i64 %27
  %.not.i.i.i7.i.i.i.3.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i, 0
  br i1 %.not.i.i.i7.i.i.i.3.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h9d34f3f86b141094E.exit"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h9d34f3f86b141094E.exit": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h976bf8cd893fa734E.exit.i.2.i"
  %28 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 64
  %29 = trunc i128 %28 to i32
  %phi.cast.i.i.2.i = uitofp i32 %29 to float
  %30 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 32
  %31 = trunc i128 %30 to i32
  %phi.cast.i.i.1.i = uitofp i32 %31 to float
  %32 = trunc i128 %_7.sroa.0.0.copyload.i.i.i.i to i32
  %phi.cast.i.i.i = uitofp i32 %32 to float
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %12) #3, !noalias !15
  %33 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 96
  %34 = trunc i128 %33 to i32
  %phi.cast.i.i.3.i = uitofp i32 %34 to float
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %7), !noalias !12
  %35 = bitcast float %phi.cast.i.i.i to i32
  %36 = bitcast float %phi.cast.i.i.1.i to i32
  %37 = bitcast float %phi.cast.i.i.2.i to i32
  %38 = bitcast float %phi.cast.i.i.3.i to i32
  %buffer.i.sroa.6.0.insert.ext = zext i32 %38 to i128
  %buffer.i.sroa.6.0.insert.shift = shl nuw i128 %buffer.i.sroa.6.0.insert.ext, 96
  %buffer.i.sroa.5.0.insert.ext = zext i32 %37 to i128
  %buffer.i.sroa.5.0.insert.shift = shl nuw nsw i128 %buffer.i.sroa.5.0.insert.ext, 64
  %buffer.i.sroa.4.0.insert.ext = zext i32 %36 to i128
  %buffer.i.sroa.4.0.insert.shift = shl nuw nsw i128 %buffer.i.sroa.4.0.insert.ext, 32
  %buffer.i.sroa.0.0.insert.ext = zext i32 %35 to i128
  %buffer.i.sroa.5.0.insert.insert = or i128 %buffer.i.sroa.5.0.insert.shift, %buffer.i.sroa.0.0.insert.ext
  %buffer.i.sroa.4.0.insert.insert = or i128 %buffer.i.sroa.5.0.insert.insert, %buffer.i.sroa.6.0.insert.shift
  %buffer.i.sroa.0.0.insert.insert = or i128 %buffer.i.sroa.4.0.insert.insert, %buffer.i.sroa.4.0.insert.shift
  ret i128 %buffer.i.sroa.0.0.insert.insert
}

; Function Attrs: nounwind nonlazybind uwtable
define void @array_cast_to_u64([4 x i64]* noalias nocapture sret dereferenceable(32) %0, [4 x i32]* noalias nocapture readonly dereferenceable(16) %x) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %tmp.i.i.i.i.i2.i.i.i.i = alloca i32, align 4
  %tmp.i.i.i.i.i.i.i.i.i = alloca i64, align 8
  %src.i.i.i.i.i.i = alloca i64, align 8
  %iter1.i.i = alloca %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", align 16
  %tmp.i.i.i.i = alloca i128, align 8
  %_10.i = alloca [4 x i32], align 4
  %1 = bitcast [4 x i32]* %x to i8*
  %2 = bitcast [4 x i32]* %_10.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %2), !noalias !23
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 4 dereferenceable(16) %2, i8* nonnull align 4 dereferenceable(16) %1, i64 16, i1 false)
  %3 = bitcast i128* %tmp.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %3) #3, !noalias !27
  %_6.i.i.i.i.i.i = ptrtoint [4 x i32]* %_10.i to i64
  %4 = bitcast i128* %tmp.i.i.i.i to [4 x i32]*
  %_6.i2.i.i.i.i.i = ptrtoint i128* %tmp.i.i.i.i to i64
  %_13.i.i.i.i.i.i = icmp ugt [4 x i32]* %_10.i, %4
  %5 = sub i64 %_6.i.i.i.i.i.i, %_6.i2.i.i.i.i.i
  %6 = sub i64 %_6.i2.i.i.i.i.i, %_6.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i, i64 %5, i64 %6
  %7 = icmp ugt i64 %diff.0.i.i.i.i.i.i, 15
  br i1 %7, label %bb3.i.i.i.lr.ph.i.i, label %bb16.i.i.i.i.i

bb16.i.i.i.i.i:                                   ; preds = %start
  call void @llvm.trap() #3, !noalias !33
  unreachable

bb3.i.i.i.lr.ph.i.i:                              ; preds = %start
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(16) %3, i8* nonnull align 4 dereferenceable(16) %2, i64 16, i1 false) #3, !noalias !33
  %_7.sroa.0.0.copyload.i.i.i.i = load i128, i128* %tmp.i.i.i.i, align 8, !noalias !27
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %3) #3, !noalias !27
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %2), !noalias !23
  %8 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i, i64 0, i32 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %8), !noalias !34
  %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i to i64*
  %9 = bitcast %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i to <2 x i64>*
  store <2 x i64> <i64 0, i64 4>, <2 x i64>* %9, align 16, !noalias !23
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i, i64 0, i32 3, i32 3
  %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i = bitcast [4 x i32]* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_idx.i to i128*
  store i128 %_7.sroa.0.0.copyload.i.i.i.i, i128* %_8.sroa.0.sroa.5.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !23
  %10 = bitcast i64* %src.i.i.i.i.i.i to i8*
  %_6.i.i.i.i.i.i.i.i.i.i.i = ptrtoint %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i to i64
  %_6.i2.i.i.i.i.i.i.i.i.i.i = ptrtoint i64* %tmp.i.i.i.i.i.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i.i.i.i.i = icmp ult i64* %tmp.i.i.i.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %11 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i2.i.i.i.i.i.i.i.i.i.i
  %12 = sub i64 %_6.i2.i.i.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i.i, i64 %11, i64 %12
  %.not.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i.i, 0
  %_6.i.i4.i.i.i.i.i.i.i.i = ptrtoint i64* %src.i.i.i.i.i.i to i64
  %13 = bitcast i32* %tmp.i.i.i.i.i2.i.i.i.i to i8*
  %_6.i2.i.i.i.i.i.i4.i.i.i.i = ptrtoint i32* %tmp.i.i.i.i.i2.i.i.i.i to i64
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.us.i.i, label %bb3.i.i.i.lr.ph.split.i.i

bb3.i.i.i.us.i.i:                                 ; preds = %bb3.i.i.i.lr.ph.i.i
  %14 = bitcast i64* %tmp.i.i.i.i.i.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %10) #3, !noalias !34
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !34
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %14) #3, !noalias !34
  call void @llvm.trap() #3, !noalias !34
  unreachable

bb3.i.i.i.lr.ph.split.i.i:                        ; preds = %bb3.i.i.i.lr.ph.i.i
  %_13.i.i.i.i.i.i.i.i.i.i = icmp ugt i64* %src.i.i.i.i.i.i, %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i
  %15 = sub i64 %_6.i.i4.i.i.i.i.i.i.i.i, %_6.i.i.i.i.i.i.i.i.i.i.i
  %16 = sub i64 %_6.i.i.i.i.i.i.i.i.i.i.i, %_6.i.i4.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i.i, i64 %15, i64 %16
  %.not1.i.i.i.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %.not1.i.i.i.i.i.i.i, label %bb3.i.i.i.us40.i.i, label %bb3.i.i.i.i.preheader.i

bb3.i.i.i.i.preheader.i:                          ; preds = %bb3.i.i.i.lr.ph.split.i.i
  %17 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 0
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %_6.i.i.i.i.i.i.i3.i.i.i.i = ptrtoint i32* %17 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %17
  %18 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %19 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.i, i64 %18, i64 %19
  %.not.i.i.i7.i.i.i.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.i, 0
  br i1 %.not.i.i.i7.i.i.i.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.i"

bb3.i.i.i.us40.i.i:                               ; preds = %bb3.i.i.i.lr.ph.split.i.i
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %10) #3, !noalias !34
  store i64 1, i64* %src.i.i.i.i.i.i, align 8, !noalias !34
  call void @llvm.trap() #3, !noalias !34
  unreachable

bb16.i.i.i.i.i.i9.i.i.i.i:                        ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.2.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.1.i", %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.i", %bb3.i.i.i.i.preheader.i
  %.lcssa.i = phi i64 [ 1, %bb3.i.i.i.i.preheader.i ], [ 2, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.i" ], [ 3, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.1.i" ], [ 4, %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.2.i" ]
  store i64 %.lcssa.i, i64* %_8.sroa.0.sroa.0.0._8.sroa.0.0..sroa_idx.sroa_cast.i, align 16, !noalias !34
  call void @llvm.trap() #3, !noalias !44
  unreachable

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.i": ; preds = %bb3.i.i.i.i.preheader.i
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %20 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 1
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %_6.i.i.i.i.i.i.i3.i.i.i.1.i = ptrtoint i32* %20 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.1.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %20
  %21 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.1.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %22 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.1.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.1.i, i64 %21, i64 %22
  %.not.i.i.i7.i.i.i.1.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.1.i, 0
  br i1 %.not.i.i.i7.i.i.i.1.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.1.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.1.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %23 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 2
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %_6.i.i.i.i.i.i.i3.i.i.i.2.i = ptrtoint i32* %23 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.2.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %23
  %24 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.2.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %25 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.2.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.2.i, i64 %24, i64 %25
  %.not.i.i.i7.i.i.i.2.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.2.i, 0
  br i1 %.not.i.i.i7.i.i.i.2.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.2.i"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.2.i": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.1.i"
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %26 = getelementptr inbounds %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>", %"std::iter::Map<std::array::IntoIter<u32, 4_usize>, [closure@/home/kadmin/projects/rust/src/test/codegen/array-map.rs:19:9: 19:21]>"* %iter1.i.i, i64 0, i32 3, i32 3, i64 3
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %_6.i.i.i.i.i.i.i3.i.i.i.3.i = ptrtoint i32* %26 to i64
  %_13.i.i.i.i.i.i.i5.i.i.i.3.i = icmp ult i32* %tmp.i.i.i.i.i2.i.i.i.i, %26
  %27 = sub i64 %_6.i.i.i.i.i.i.i3.i.i.i.3.i, %_6.i2.i.i.i.i.i.i4.i.i.i.i
  %28 = sub i64 %_6.i2.i.i.i.i.i.i4.i.i.i.i, %_6.i.i.i.i.i.i.i3.i.i.i.3.i
  %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i = select i1 %_13.i.i.i.i.i.i.i5.i.i.i.3.i, i64 %27, i64 %28
  %.not.i.i.i7.i.i.i.3.i = icmp eq i64 %diff.0.i.i.i.i.i.i.i6.i.i.i.3.i, 0
  br i1 %.not.i.i.i7.i.i.i.3.i, label %bb16.i.i.i.i.i.i9.i.i.i.i, label %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h33ba33685550836aE.exit"

"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h33ba33685550836aE.exit": ; preds = %"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map21MapGuard$LT$T$C$_$GT$4push17h77486a408cc0afeaE.exit.i.2.i"
  %29 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 64
  %30 = trunc i128 %29 to i64
  %phi.cast.i.i.2.i = and i64 %30, 4294967295
  %31 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 32
  %32 = insertelement <2 x i128> undef, i128 %_7.sroa.0.0.copyload.i.i.i.i, i32 0
  %33 = insertelement <2 x i128> %32, i128 %31, i32 1
  %34 = trunc <2 x i128> %33 to <2 x i64>
  %35 = and <2 x i64> %34, <i64 4294967295, i64 4294967295>
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %13) #3, !noalias !37
  %36 = lshr i128 %_7.sroa.0.0.copyload.i.i.i.i, 96
  %37 = trunc i128 %36 to i64
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %8), !noalias !34
  %38 = bitcast [4 x i64]* %0 to <2 x i64>*
  store <2 x i64> %35, <2 x i64>* %38, align 8, !noalias !45
  %buffer.i.sroa.5.0..sroa_idx5 = getelementptr inbounds [4 x i64], [4 x i64]* %0, i64 0, i64 2
  store i64 %phi.cast.i.i.2.i, i64* %buffer.i.sroa.5.0..sroa_idx5, align 8, !noalias !45
  %buffer.i.sroa.6.0..sroa_idx7 = getelementptr inbounds [4 x i64], [4 x i64]* %0, i64 0, i64 3
  store i64 %37, i64* %buffer.i.sroa.6.0..sroa_idx7, align 8, !noalias !45
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
!3 = distinct !{!3, !4, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h9d34f3f86b141094E: %self"}
!4 = distinct !{!4, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h9d34f3f86b141094E"}
!5 = !{!6, !8, !10, !3}
!6 = distinct !{!6, !7, !"_ZN4core3mem14transmute_copy17hff2bea9b8ad70982E: %src"}
!7 = distinct !{!7, !"_ZN4core3mem14transmute_copy17hff2bea9b8ad70982E"}
!8 = distinct !{!8, !9, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17hebea8eb671b01b6aE: %iter"}
!9 = distinct !{!9, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17hebea8eb671b01b6aE"}
!10 = distinct !{!10, !9, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17hebea8eb671b01b6aE: %array"}
!11 = !{!8, !3}
!12 = !{!13, !3}
!13 = distinct !{!13, !14, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17hb5ead863180d260dE: %iter"}
!14 = distinct !{!14, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17hb5ead863180d260dE"}
!15 = !{!16, !18, !20, !13, !3}
!16 = distinct !{!16, !17, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17h27bb0d41c16531bdE: %self"}
!17 = distinct !{!17, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17h27bb0d41c16531bdE"}
!18 = distinct !{!18, !19, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h6c26493545893eb3E: %_1"}
!19 = distinct !{!19, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h6c26493545893eb3E"}
!20 = distinct !{!20, !21, !"_ZN4core6option15Option$LT$T$GT$3map17h6a6254d4699660d7E: %f"}
!21 = distinct !{!21, !"_ZN4core6option15Option$LT$T$GT$3map17h6a6254d4699660d7E"}
!22 = !{!18, !20, !13, !3}
!23 = !{!24, !26}
!24 = distinct !{!24, !25, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h33ba33685550836aE: argument 0"}
!25 = distinct !{!25, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h33ba33685550836aE"}
!26 = distinct !{!26, !25, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map17h33ba33685550836aE: %self"}
!27 = !{!28, !30, !32, !24, !26}
!28 = distinct !{!28, !29, !"_ZN4core3mem14transmute_copy17hff2bea9b8ad70982E: %src"}
!29 = distinct !{!29, !"_ZN4core3mem14transmute_copy17hff2bea9b8ad70982E"}
!30 = distinct !{!30, !31, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17hebea8eb671b01b6aE: %iter"}
!31 = distinct !{!31, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17hebea8eb671b01b6aE"}
!32 = distinct !{!32, !31, !"_ZN4core5array4iter21IntoIter$LT$T$C$_$GT$3new17hebea8eb671b01b6aE: %array"}
!33 = !{!30, !24, !26}
!34 = !{!35, !24, !26}
!35 = distinct !{!35, !36, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17h8eb0f5bf764aad08E: %iter"}
!36 = distinct !{!36, !"_ZN4core5array40_$LT$impl$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3map12map_guard_fn17h8eb0f5bf764aad08E"}
!37 = !{!38, !40, !42, !35, !24, !26}
!38 = distinct !{!38, !39, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17h27bb0d41c16531bdE: %self"}
!39 = distinct !{!39, !"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$16assume_init_read17h27bb0d41c16531bdE"}
!40 = distinct !{!40, !41, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h6c26493545893eb3E: %_1"}
!41 = distinct !{!41, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h6c26493545893eb3E"}
!42 = distinct !{!42, !43, !"_ZN4core6option15Option$LT$T$GT$3map17h6a6254d4699660d7E: %f"}
!43 = distinct !{!43, !"_ZN4core6option15Option$LT$T$GT$3map17h6a6254d4699660d7E"}
!44 = !{!40, !42, !35, !24, !26}
!45 = !{!26}
