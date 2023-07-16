 llvm
; before
define void @foo::hac36ebd8661e5cd1aj::v0.0({ void (i8*)**, i8* }*) #4 {
entry-block:
  %x = alloca { void (i8*)**, i8* }
  %trait_callee = alloca { void (i8*)**, i8* }
  %1 = bitcast { void (i8*)**, i8* }* %0 to i8*
  %2 = bitcast { void (i8*)**, i8* }* %x to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %1, i64 16, i32 8, i1 false)
  %3 = bitcast { void (i8*)**, i8* }* %x to i8*
  %4 = bitcast { void (i8*)**, i8* }* %trait_callee to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %3, i64 16, i32 8, i1 false)
  %5 = getelementptr inbounds { void (i8*)**, i8* }* %trait_callee, i32 0, i32 1
  %6 = load i8** %5
  %7 = getelementptr inbounds { void (i8*)**, i8* }* %trait_callee, i32 0, i32 0
  %8 = bitcast void (i8*)*** %7 to [1 x i8**]**
  %9 = load [1 x i8**]** %8
  %10 = getelementptr inbounds [1 x i8**]* %9, i32 0, i32 1
  %11 = load i8*** %10
  %12 = bitcast i8** %11 to void (i8*)*
  call void %12(i8* %6)
  ret void
}
