llvm
; playground::zeroize_b8
; Function Attrs: nounwind nonlazybind uwtable
define void @_ZN10playground10zeroize_b817h04a795eb43fb6ecfE(%B8* align 8 dereferenceable(8) %bytes) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_5.sroa.0.i = alloca i64, align 8
  %_5.sroa.0.i.0.sroa_cast = bitcast i64* %_5.sroa.0.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %_5.sroa.0.i.0.sroa_cast)
  store i64 0, i64* %_5.sroa.0.i, align 8
  %_5.sroa.0.0..sroa_cast.i = bitcast %B8* %bytes to i64*
  ; the volatile load occurs in the next line
  ; this shouldn't be volatile: the store later is volatile. That should be enough.
  %_5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0.copyload.i = load volatile i64, i64* %_5.sroa.0.i, align 8
  ; the store in the next line is volatile, which is what we want
  store volatile i64 %_5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0.copyload.i, i64* %_5.sroa.0.0..sroa_cast.i, align 8
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %_5.sroa.0.i.0.sroa_cast)
  ret void
}
