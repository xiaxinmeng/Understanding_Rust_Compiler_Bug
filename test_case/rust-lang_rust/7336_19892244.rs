 llvm
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
  %tmp.i = alloca { i64, %tydesc*, {}*, {}*, { i64* } }, align 8
  %state.i = alloca i64, align 8
  %0 = bitcast i64* %state.i to i8*
  call void @llvm.lifetime.start(i64 8, i8* %0)
  store i64 543210, i64* %state.i, align 8
  %1 = bitcast { i64, %tydesc*, {}*, {}*, { i64* } }* %tmp.i to i8*
  call void @llvm.lifetime.start(i64 40, i8* %1)
  %tmp7.i = bitcast { i64, %tydesc*, {}*, {}*, { i64* } }* %tmp.i to { i64, %tydesc*, i8*, i8*, i8 }*
  %tmp8.i = getelementptr inbounds { i64, %tydesc*, {}*, {}*, { i64* } }* %tmp.i, i64 0, i32 0
  store i64 305419896, i64* %tmp8.i, align 8
  %tmp9.i = getelementptr inbounds { i64, %tydesc*, {}*, {}*, { i64* } }* %tmp.i, i64 0, i32 4, i32 0
  store i64* %state.i, i64** %tmp9.i, align 8
  call void @llvm.lifetime.end(i64 40, i8* %1)
  %tmp18.i = call {} @_ZN4test4anon4anon12expr_fn_2956E({ i64, %tydesc*, i8*, i8*, i8 }* %tmp7.i)
  call void @llvm.lifetime.end(i64 8, i8* %0)
  ret void
}
*** IR Dump After Dead Store Elimination ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
  %tmp.i = alloca { i64, %tydesc*, {}*, {}*, { i64* } }, align 8
  %state.i = alloca i64, align 8
  %0 = bitcast i64* %state.i to i8*
  call void @llvm.lifetime.start(i64 8, i8* %0)
  %1 = bitcast { i64, %tydesc*, {}*, {}*, { i64* } }* %tmp.i to i8*
  call void @llvm.lifetime.start(i64 40, i8* %1)
  %tmp7.i = bitcast { i64, %tydesc*, {}*, {}*, { i64* } }* %tmp.i to { i64, %tydesc*, i8*, i8*, i8 }*
  call void @llvm.lifetime.end(i64 40, i8* %1)
  %tmp18.i = call {} @_ZN4test4anon4anon12expr_fn_2956E({ i64, %tydesc*, i8*, i8*, i8 }* %tmp7.i)
  call void @llvm.lifetime.end(i64 8, i8* %0)
  ret void
}
