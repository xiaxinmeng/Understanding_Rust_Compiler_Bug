llvm
; test::slow
; Function Attrs: nounwind readnone uwtable
define void @_ZN4test4slow17hb13ef2f0fb17248eE() unnamed_addr #0 {
start:
  %_5.i = alloca [4096 x i8], align 1
  %buf = alloca [4096 x i8], align 1
  %buf.0.sroa_idx1 = getelementptr inbounds [4096 x i8], [4096 x i8]* %buf, i64 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 4096, i8* nonnull %buf.0.sroa_idx1)
  %_5.0.sroa_idx2.i = getelementptr inbounds [4096 x i8], [4096 x i8]* %_5.i, i64 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 4096, i8* nonnull %_5.0.sroa_idx2.i)
  call void @llvm.memset.p0i8.i64(i8* nonnull align 1 %_5.0.sroa_idx2.i, i8 0, i64 4096, i1 false)
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 1 %buf.0.sroa_idx1, i8* nonnull align 1 %_5.0.sroa_idx2.i, i64 4096, i1 true) #2, !noalias !0
  call void @llvm.lifetime.end.p0i8(i64 4096, i8* nonnull %_5.0.sroa_idx2.i)
  call void @llvm.lifetime.end.p0i8(i64 4096, i8* nonnull %buf.0.sroa_idx1)
  ret void
}
