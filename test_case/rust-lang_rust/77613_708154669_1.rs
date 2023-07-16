llvm
define void @_ZN3lib1g17h19b6423a2d95f243E(i1 zeroext %clip) unnamed_addr #0 {
start:
  %item.sroa.2 = alloca [22 x i64], align 8
  %_5 = alloca %DI, align 8
  br i1 %clip, label %bb4, label %bb1

bb1:                                              ; preds = %start
  %0 = bitcast %DI* %_5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 184, i8* nonnull %0)
  %item.sroa.0.0..sroa_idx = getelementptr inbounds %DI, %DI* %_5, i64 0, i32 0, i64 0
  store i64 0, i64* %item.sroa.0.0..sroa_idx, align 8
  %item.sroa.2.0..sroa_idx1 = getelementptr inbounds %DI, %DI* %_5, i64 0, i32 1, i32 2
  %item.sroa.2.0..sroa_cast = bitcast [22 x i64]* %item.sroa.2.0..sroa_idx1 to i8*
  %item.sroa.2.0.sroa_cast = bitcast [22 x i64]* %item.sroa.2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(176) %item.sroa.2.0..sroa_cast, i8* nonnull align 8 dereferenceable(176) %item.sroa.2.0.sroa_cast, i64 176, i1 false)
  call fastcc void @_ZN3lib7do_item17h4ec77638412d024eE(%DI* noalias nonnull readonly align 8 dereferenceable(184) %_5)
  call void @llvm.lifetime.end.p0i8(i64 184, i8* nonnull %0)
  br label %bb4

bb4:                                              ; preds = %start, %bb1
  ret void
}
