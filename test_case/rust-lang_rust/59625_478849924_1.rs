LLVM
; <&mut core::ffi::VaListInner as core::ffi::VaListMethods>::raw_copy
; Function Attrs: alwaysinline nonlazybind uwtable
define internal align 8 dereferenceable(24) %"core::ffi::VaListInner"* @"_ZN79_$LT$$RF$mut$u20$core..ffi..VaListInner$u20$as$u20$core..ffi..VaListMethods$GT$8raw_copy17h78377ab98854a549E"(%"core::ffi::VaListInner"** noalias readonly align 8 dereferenceable(8) %self) unnamed_addr #1 {
start:
  %0 = alloca %"core::ffi::VaListInner", align 8
  %tmp_ret = alloca %"core::ffi::VaListInner"*, align 8
  %1 = bitcast %"core::ffi::VaListInner"** %tmp_ret to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* %1)
  %2 = load %"core::ffi::VaListInner"*, %"core::ffi::VaListInner"** %self, align 8
  store %"core::ffi::VaListInner"* %0, %"core::ffi::VaListInner"** %tmp_ret, align 8
  %3 = bitcast %"core::ffi::VaListInner"* %0 to i8*
  %4 = bitcast %"core::ffi::VaListInner"* %2 to i8*
  call void @llvm.va_copy(i8* %3, i8* %4)
  %5 = load %"core::ffi::VaListInner"*, %"core::ffi::VaListInner"** %tmp_ret, align 8, !nonnull !3
  %6 = bitcast %"core::ffi::VaListInner"** %tmp_ret to i8*
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %6)
  br label %bb1

bb1:                                              ; preds = %start
  ret %"core::ffi::VaListInner"* %5
}
