 llvm
*** IR Dump After Deduce function attributes ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
  %tmp.i4 = alloca {}, align 8
  %tmp.i = alloca { i64, %tydesc*, {}*, {}*, { i64** } }, align 8
  %0 = bitcast { i64, %tydesc*, {}*, {}*, { i64** } }* %tmp.i to i8*
  call void @llvm.lifetime.start(i64 40, i8* %0)
  %tmp6.i = bitcast { i64, %tydesc*, {}*, {}*, { i64** } }* %tmp.i to { i64, %tydesc*, i8*, i8*, i8 }*
  call void @llvm.lifetime.end(i64 40, i8* %0)
  %1 = bitcast {}* %tmp.i4 to i8*
  call void @llvm.lifetime.start(i64 0, i8* %1)
  %tmp2.i = getelementptr inbounds { i64, %tydesc*, i8*, i8*, i8 }* %tmp6.i, i64 0, i32 4
  %tmp3.i = bitcast i8* %tmp2.i to i64***
  %tmp4.i = load i64*** %tmp3.i, align 8
  %tmp5.i = load i64** %tmp4.i, align 8
  %tmp6.i5 = load i64* %tmp5.i, align 8
  %tmp7.i = add i64 %tmp6.i5, 1
  store i64 %tmp7.i, i64* %tmp5.i, align 8
  %tmp8.i = load {}* %tmp.i4, align 8
  %2 = bitcast {}* %tmp.i4 to i8*
  call void @llvm.lifetime.end(i64 0, i8* %2)
  ret void
}
*** IR Dump After SROA ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
  %tmp.i4 = alloca {}, align 8
  %0 = bitcast {}* %tmp.i4 to i8*
  call void @llvm.lifetime.start(i64 0, i8* %0)
  %tmp5.i = load i64** undef, align 8
  %tmp6.i5 = load i64* %tmp5.i, align 8
  %tmp7.i = add i64 %tmp6.i5, 1
  store i64 %tmp7.i, i64* %tmp5.i, align 8
  %tmp8.i = load {}* %tmp.i4, align 8
  %1 = bitcast {}* %tmp.i4 to i8*
  call void @llvm.lifetime.end(i64 0, i8* %1)
  ret void
}
