
; Function Attrs: nounwind nonlazybind
define void @_ZN6test111g17h45fb2453d8cc5713E([80 x i64]* noalias nocapture sret([80 x i64]) dereferenceable(640) %0, {}* nonnull align 1 %1, [3 x i64]* noalias nocapture readonly align 8 dereferenceable(24) %2) unnamed_addr #0 {
  %4 = alloca [40 x i64], align 8
  %5 = alloca [40 x i64], align 8
  %6 = bitcast [40 x i64]* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 320, i8* nonnull %6)
  %7 = getelementptr inbounds [3 x i64], [3 x i64]* %2, i64 0, i64 3
  %8 = bitcast i64* %7 to void ([40 x i64]*, {}*)**
  %9 = load void ([40 x i64]*, {}*)*, void ([40 x i64]*, {}*)** %8, align 8, !invariant.load !2, !nonnull !2
  %10 = bitcast [80 x i64]* %0 to [40 x i64]*
  call void %9([40 x i64]* noalias nocapture nonnull sret([40 x i64]) dereferenceable(320) %10, {}* nonnull align 1 %1) #5
  %11 = bitcast [40 x i64]* %4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 320, i8* nonnull %11)
  call void %9([40 x i64]* noalias nocapture nonnull sret([40 x i64]) dereferenceable(320) %4, {}* nonnull align 1 %1) #5
  %12 = bitcast [80 x i64]* %0 to i8*
  %13 = getelementptr i8, i8* %12, i64 320
  call void @llvm.memset.p0i8.i64(i8* align 8 %13, i8 0, i64 320, i1 false)
  %14 = getelementptr inbounds [80 x i64], [80 x i64]* %0, i64 0, i64 40
  %15 = bitcast i64* %14 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(320) %15, i8* nonnull align 8 dereferenceable(320) %11, i64 320, i1 false) #5
  call void @llvm.lifetime.end.p0i8(i64 320, i8* nonnull %11)
  call void @llvm.lifetime.end.p0i8(i64 320, i8* nonnull %6)
  ret void
}
