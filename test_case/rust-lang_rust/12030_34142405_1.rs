 llvm
; before
define internal void @foo({ void (i8*)**, i8* }*) unnamed_addr #4 {
entry-block:
  %x = alloca { void (i8*)**, i8* }
  %y = alloca { void (i8*)**, i8* }
  %__auto_borrow_obj = alloca { void (i8*)**, i8* }
  %1 = bitcast { void (i8*)**, i8* }* %0 to i8*
  %2 = bitcast { void (i8*)**, i8* }* %x to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %1, i64 16, i32 8, i1 false)
  %3 = getelementptr inbounds { void (i8*)**, i8* }* %x, i32 0, i32 0
  %4 = load void (i8*)*** %3
  %5 = getelementptr inbounds { void (i8*)**, i8* }* %__auto_borrow_obj, i32 0, i32 0
  store void (i8*)** %4, void (i8*)*** %5
  %6 = getelementptr inbounds { void (i8*)**, i8* }* %x, i32 0, i32 1
  %7 = load i8** %6
  %8 = getelementptr inbounds { void (i8*)**, i8* }* %__auto_borrow_obj, i32 0, i32 1
  store i8* %7, i8** %8
  %9 = bitcast { void (i8*)**, i8* }* %__auto_borrow_obj to i8*
  %10 = bitcast { void (i8*)**, i8* }* %y to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %10, i8* %9, i64 16, i32 8, i1 false)
  call void @"~std::clone::Clone.Send::glue_drop"({ void (i8*)**, i8* }* %x)
  ret void
}
