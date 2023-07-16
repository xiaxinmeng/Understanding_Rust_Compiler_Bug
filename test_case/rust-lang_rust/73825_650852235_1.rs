llvm
define i32 @_ZN10playground3foo17hf1f5714ed87b6704E(i64 %x) unnamed_addr #0 {
start:
  %base = alloca [5 x i32], align 16
  %0 = bitcast [5 x i32]* %base to i8*
  call void @llvm.lifetime.start.p0i8(i64 20, i8* nonnull %0)
  %1 = bitcast [5 x i32]* %base to <4 x i32>*
  store <4 x i32> <i32 1, i32 3, i32 4, i32 5>, <4 x i32>* %1, align 16
  %2 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i64 0, i64 4
  store i32 2, i32* %2, align 16
  %_3 = urem i64 %x, 5
  %3 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i64 0, i64 %_3
  %4 = load i32, i32* %3, align 4
  call void @llvm.lifetime.end.p0i8(i64 20, i8* nonnull %0)
  ret i32 %4
}
