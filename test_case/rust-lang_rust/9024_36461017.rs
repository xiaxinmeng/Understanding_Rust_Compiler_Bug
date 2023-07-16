 rust
; Function Attrs: uwtable
define void @_ZN6addf3220h7431f4d506cb9520daa4v0.0E([3 x float]* noalias nocapture sret, [3 x float]* noalias nocapture readonly, [3 x float]* noalias nocapture readonly) unnamed_addr #1 {
entry-block:
  %s = alloca [3 x float], align 4
  %3 = bitcast [3 x float]* %s to i8*
  call void @llvm.memset.p0i8.i64(i8* %3, i8 0, i64 12, i32 4, i1 false)
  br label %next7

next7:                                            ; preds = %next7, %entry-block
  %4 = phi i64 [ 0, %entry-block ], [ %5, %next7 ]
  %5 = add i64 %4, 1
  %6 = getelementptr inbounds [3 x float]* %2, i64 0, i64 %4
  %7 = getelementptr inbounds [3 x float]* %1, i64 0, i64 %4
  %8 = load float* %6, align 4
  %9 = load float* %7, align 4
  %10 = fadd float %8, %9
  %11 = getelementptr inbounds [3 x float]* %s, i64 0, i64 %4
  store float %10, float* %11, align 4
  %12 = icmp slt i64 %5, 3
  br i1 %12, label %next7, label %join9

join9:                                            ; preds = %next7
  %13 = bitcast [3 x float]* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %13, i8* %3, i64 12, i32 4, i1 false)
  ret void
}
