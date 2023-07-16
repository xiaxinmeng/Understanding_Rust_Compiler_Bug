 llvm
; after
define internal void @foo({ void (i8*)**, i8* }*) unnamed_addr #4 {
entry-block:
  %y = alloca { void (i8*)**, i8* }
  %1 = bitcast { void (i8*)**, i8* }* %0 to i8*
  %2 = bitcast { void (i8*)**, i8* }* %y to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %1, i64 16, i32 8, i1 false)
  call void @"~std::clone::Clone.Send::glue_drop"({ void (i8*)**, i8* }* %0)
  ret void
}
