llvm
define zeroext i1 @slow_iters::tests::iter_any::h9958878595c8e553() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %::sroa.0.i.i.i.i = alloca i8, align 1
  %::sroa.3.i.i.i.i = alloca i8, align 1
  call void @llvm.lifetime.start.p0i8(i64 1, i8* nonnull %::sroa.0.i.i.i.i)
  call void @llvm.lifetime.start.p0i8(i64 1, i8* nonnull %::sroa.3.i.i.i.i)
  store i8 1, i8* %::sroa.0.i.i.i.i, align 1
  store i8 0, i8* %::sroa.3.i.i.i.i, align 1
  %::sroa.0.i.i.i.i.0._0.sroa.0.i.i.i.i.0._0.sroa.0.i.i.i.i.0._0.sroa.0.i.i.i.0._0.sroa.0.i.i.0._0.sroa.0.i.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i.i.i.i = load i8, i8* %::sroa.0.i.i.i.i, align 1
  %::sroa.3.i.i.i.i.0._0.sroa.3.i.i.i.i.0._0.sroa.3.i.i.i.i.0._0.sroa.3.i.i.i.0._0.sroa.3.i.i.0._0.sroa.3.i.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i.i.i.i = load i8, i8* %::sroa.3.i.i.i.i, align 1
  call void @llvm.lifetime.end.p0i8(i64 1, i8* nonnull %::sroa.0.i.i.i.i)
  call void @llvm.lifetime.end.p0i8(i64 1, i8* nonnull %::sroa.3.i.i.i.i)
  %cond.i.i.i = icmp eq i8 %::sroa.0.i.i.i.i.0._0.sroa.0.i.i.i.i.0._0.sroa.0.i.i.i.i.0._0.sroa.0.i.i.i.0._0.sroa.0.i.i.0._0.sroa.0.i.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i.i.i.i, 0
  %0 = and i8 %::sroa.3.i.i.i.i.0._0.sroa.3.i.i.i.i.0._0.sroa.3.i.i.i.i.0._0.sroa.3.i.i.i.0._0.sroa.3.i.i.0._0.sroa.3.i.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i.i.i.i, 1
  %phitmp8 = icmp eq i8 %0, 0
  %::0.i.i.i = select i1 %cond.i.i.i, i1 false, i1 %phitmp8
  ret i1 %::0.i.i.i
}
