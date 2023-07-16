llvm
define void @f(ptr noalias nocapture noundef readonly dereferenceable(256) %a, ptr nocapture noundef nonnull readonly %b) unnamed_addr #0 {
start:
  %0 = alloca [32 x i64], align 8
  %1 = alloca [32 x i64], align 8
  call void @llvm.lifetime.start.p0(i64 256, ptr nonnull %1)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(256) %1, ptr noundef nonnull align 8 dereferenceable(256) %a, i64 256, i1 false)
  call void @llvm.lifetime.start.p0(i64 256, ptr nonnull %0)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(256) %0, ptr noundef nonnull align 8 dereferenceable(256) %a, i64 256, i1 false)
  call void %b(ptr noalias nocapture noundef nonnull dereferenceable(256) %1, ptr noalias nocapture noundef nonnull dereferenceable(256) %0)
  call void @llvm.lifetime.end.p0(i64 256, ptr nonnull %1)
  call void @llvm.lifetime.end.p0(i64 256, ptr nonnull %0)
  ret void
}
