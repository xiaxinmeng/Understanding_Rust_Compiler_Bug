
  %37 = getelementptr inbounds %Biquad, %Biquad* %35, i64 1
  %38 = bitcast %Biquad* %37 to i8*
  call void @llvm.memset.p0i8.i64(i8* %38, i8 0, i64 72, i32 8, i1 false)
  %39 = getelementptr inbounds %Biquad, %Biquad* %37, i64 1
  %40 = bitcast %Biquad* %39 to i8*
  call void @llvm.memset.p0i8.i64(i8* %40, i8 0, i64 72, i32 8, i1 false)
  %41 = getelementptr inbounds %Biquad, %Biquad* %39, i64 1
  %42 = bitcast %Biquad* %41 to i8*
  call void @llvm.memset.p0i8.i64(i8* %42, i8 0, i64 72, i32 8, i1 false)
  %43 = getelementptr inbounds %Biquad, %Biquad* %41, i64 1
  %44 = bitcast %Biquad* %43 to i8*
