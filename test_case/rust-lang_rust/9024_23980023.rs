
; ModuleID = 'foo.rc'

%tydesc = type { i64, i64, void ({}*, i8*)*, void ({}*, i8*)*, void ({}*, i8*)*, void ({}*, i8*)*, i64, { i8*, i64 } }

declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture, i8* nocapture readonly, i64, i32, i1)

declare void @llvm.memset.p0i8.i64(i8* nocapture, i8, i64, i32, i1)

define void @_ZN3add16_d492e97021e66524v0.0E([3 x float]* nocapture noalias, {
        i64, %tydesc*, i8*, i8*, i8 }* nocapture readnone, [3 x float]*
        nocapture noalias readonly, [3 x float]* nocapture noalias readonly) {
"function top level":
  %s = alloca [3 x float], align 4
  %4 = bitcast [3 x float]* %s to i8*
  call void @llvm.memset.p0i8.i64(i8* %4, i8 0, i64 12, i32 4, i1 false)
  br label %match_else

match_else:                                       ; preds = %"function top level", %next6
  %.sroa.013.0.load1922 = phi i64 [ 0, %"function top level" ], [ %8, %next6 ]
  %5 = shl i64 %.sroa.013.0.load1922, 2
  %6 = icmp ugt i64 %.sroa.013.0.load1922, 2
  br i1 %6, label %cond, label %next6

match_case:                                       ; preds = %next6
  %7 = bitcast [3 x float]* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %7, i8* %4, i64 12, i32 4, i1 false)
  ret void

cond:                                             ; preds = %match_else
    ret void

next6:                                            ; preds = %match_else
  %8 = add i64 %.sroa.013.0.load1922, 1
  %9 = getelementptr inbounds [3 x float]* %3, i64 0, i64 %.sroa.013.0.load1922
  %10 = getelementptr inbounds [3 x float]* %2, i64 0, i64 %.sroa.013.0.load1922
  %11 = load float* %9, align 4
  %12 = load float* %10, align 4
  %13 = fadd float %11, %12
  %14 = getelementptr inbounds [3 x float]* %s, i64 0, i64 %.sroa.013.0.load1922
  store float %13, float* %14, align 4
  %15 = icmp slt i64 %8, 3
  br i1 %15, label %match_else, label %match_case
}
