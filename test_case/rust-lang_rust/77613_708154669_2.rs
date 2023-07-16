llvm
define void @_ZN3lib1g17h19b6423a2d95f243E(i1 zeroext %clip) unnamed_addr #0 {
start:
  %_8.sroa.4 = alloca [22 x i64], align 8
  %_7 = alloca %DI, align 8
  %item.sroa.4 = alloca [22 x i64], align 8
  br i1 %clip, label %bb4, label %bb1

bb1:                                              ; preds = %start
  %item.sroa.4.0.sroa_cast = bitcast [22 x i64]* %item.sroa.4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 176, i8* nonnull %item.sroa.4.0.sroa_cast)
  %0 = bitcast %DI* %_7 to i8*
  call void @llvm.lifetime.start.p0i8(i64 184, i8* nonnull %0)
  %_8.sroa.4.0.sroa_cast10 = bitcast [22 x i64]* %_8.sroa.4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 176, i8* nonnull %_8.sroa.4.0.sroa_cast10)
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(176) %_8.sroa.4.0.sroa_cast10, i8* nonnull align 8 dereferenceable(176) %item.sroa.4.0.sroa_cast, i64 176, i1 false)
  %_8.sroa.0.0..sroa_idx = getelementptr inbounds %DI, %DI* %_7, i64 0, i32 0, i64 0
  store i64 0, i64* %_8.sroa.0.0..sroa_idx, align 8
  %_8.sroa.4.0..sroa_idx7 = getelementptr inbounds %DI, %DI* %_7, i64 0, i32 1, i32 2
  %_8.sroa.4.0..sroa_cast = bitcast [22 x i64]* %_8.sroa.4.0..sroa_idx7 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(176) %_8.sroa.4.0..sroa_cast, i8* nonnull align 8 dereferenceable(176) %_8.sroa.4.0.sroa_cast10, i64 176, i1 false)
  call void @llvm.lifetime.end.p0i8(i64 176, i8* nonnull %_8.sroa.4.0.sroa_cast10)
  call fastcc void @_ZN3lib7do_item17h4ec77638412d024eE(%DI* noalias nonnull readonly align 8 dereferenceable(184) %_7)
  call void @llvm.lifetime.end.p0i8(i64 184, i8* nonnull %0)
  call void @llvm.lifetime.end.p0i8(i64 176, i8* nonnull %item.sroa.4.0.sroa_cast)
  br label %bb4

bb4:                                              ; preds = %start, %bb1
  ret void
}
