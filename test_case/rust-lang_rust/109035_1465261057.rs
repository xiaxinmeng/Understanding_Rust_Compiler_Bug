ll
define noundef i32 @demo(ptr noalias noundef readonly align 4 dereferenceable(4) %x) unnamed_addr #0 {
  %tmp = alloca i32, align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr %tmp)
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %tmp, ptr align 4 %x, i64 4, i1 false)
  %self = load i32, ptr %tmp, align 4
  call void @llvm.lifetime.end.p0(i64 4, ptr %tmp)
  ret i32 %self
}
