llvm
  %val1 = alloca %"ThreeSlices<'_>", align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %val1)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %val1, ptr noundef nonnull align 8 dereferenceable(48) %val, i64 48, i1 false)
  %0 = call noundef i32 @example::sum(ptr noalias noundef nonnull readonly align 8 dereferenceable(48) %val1)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %val1)
  ret i32 %0
