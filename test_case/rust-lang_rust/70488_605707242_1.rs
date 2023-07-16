llvm
; playground::iter
; Function Attrs: noinline noreturn nounwind nonlazybind uwtable
define internal fastcc void @_ZN10playground4iter17h1130617e37f08519E() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_3 = alloca %"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>", align 8
  %_3.0.sroa_cast = bitcast %"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>"* %_3 to i8*
  %_3.8.sroa_idx = getelementptr inbounds %"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>", %"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>"* %_3, i64 0, i32 1, i32 1
  br label %bb1

bb1:                                              ; preds = %bb1, %start
  call void @llvm.lifetime.start.p0i8(i64 12, i8* nonnull %_3.0.sroa_cast)
  store i8 0, i8* %_3.8.sroa_idx, align 8, !alias.scope !2
  %_3.0. = load volatile %"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>", %"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>"* %_3, align 8, !alias.scope !8, !noalias !11
  call void @llvm.lifetime.end.p0i8(i64 12, i8* nonnull %_3.0.sroa_cast)
  br label %bb1
}
