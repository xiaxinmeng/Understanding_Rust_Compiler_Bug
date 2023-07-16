
define void @_ZN5test24test17h0962739e00df972bE() unnamed_addr #0 {
start:
  %buffer2 = alloca [256 x i16], align 2
  %buffer = alloca [512 x i16], align 2
  %0 = bitcast [512 x i16]* %buffer to i8*
  call void @llvm.lifetime.start.p0i8(i64 1024, i8* nonnull %0)
  %1 = bitcast [256 x i16]* %buffer2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 512, i8* nonnull %1)
  %2 = getelementptr inbounds [512 x i16], [512 x i16]* %buffer, i64 0, i64 0
  call void @fill(i16* nonnull %2)
  %3 = getelementptr inbounds [256 x i16], [256 x i16]* %buffer2, i64 0, i64 0
  call void @fill(i16* nonnull %3)
  call void @llvm.lifetime.end.p0i8(i64 512, i8* nonnull %1)
  call void @llvm.lifetime.end.p0i8(i64 1024, i8* nonnull %0)
  ret void
}
