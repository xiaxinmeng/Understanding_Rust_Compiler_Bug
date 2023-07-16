 ll
define internal void @_ZN5panic20hbd7799a4d53a7c18eaaE() unnamed_addr #0 {
entry-block:
  %0 = tail call i8* @je_mallocx(i64 4, i32 0)
  %1 = icmp eq i8* %0, null
  br i1 %1, label %then-block-132-.i.i, label %normal-return

then-block-132-.i.i:                              ; preds = %entry-block
  tail call void @_ZN3oom20h5ea7b8934ca1bc543ZaE()
  unreachable

normal-return:                                    ; preds = %entry-block
  %2 = bitcast i8* %0 to i32*
  store i32 1, i32* %2, align 4
  invoke fastcc void @_ZN12should_panic20hfe6f725236e48244xaaE(i32* noalias dereferenceable(4) %2)
          to label %normal-return1 unwind label %unwind_custom_

unwind_custom_:                                   ; preds = %normal-return
  %3 = landingpad { i8*, i32 } personality i32 (i32, i32, i64, %"1.std::rt::libunwind::_Unwind_Exception"*, %"1.std::rt::libunwind::_Unwind_Context"*)* @rust_eh_personality
          cleanup
  %4 = icmp eq i8* %0, inttoptr (i64 2097865012304223517 to i8*)
  br i1 %4, label %"_ZN14Box$LT$u32$GT$9drop.118117h48c45852b94c565eE.exit", label %cond.i

cond.i:                                           ; preds = %unwind_custom_
  tail call void @je_sdallocx(i8* %0, i64 4, i32 0)
  br label %"_ZN14Box$LT$u32$GT$9drop.118117h48c45852b94c565eE.exit"

"_ZN14Box$LT$u32$GT$9drop.118117h48c45852b94c565eE.exit": ; preds = %unwind_custom_, %cond.i
  resume { i8*, i32 } %3

normal-return1:                                   ; preds = %normal-return
  ret void
}

; Function Attrs: noinline noreturn uwtable
define internal fastcc void @_ZN12should_panic20hfe6f725236e48244xaaE(i32* noalias dereferenceable(4)) unnamed_addr #1 {
entry-block:
  %const = alloca %str_slice, align 8
  %1 = bitcast %str_slice* %const to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %1, i8* bitcast (%str_slice* @const1222 to i8*), i64 16, i32 8, i1 false)
  invoke fastcc void @_ZN2rt6unwind12begin_unwind20h8221535833087833418E(%str_slice* noalias nocapture dereferenceable(16) %const)
          to label %normal-return unwind label %unwind_custom_

normal-return:                                    ; preds = %entry-block
  call void @llvm.lifetime.end(i64 16, i8* %1)
  unreachable

unwind_custom_:                                   ; preds = %entry-block
  %2 = landingpad { i8*, i32 } personality i32 (i32, i32, i64, %"1.std::rt::libunwind::_Unwind_Exception"*, %"1.std::rt::libunwind::_Unwind_Context"*)* @rust_eh_personality
          cleanup
  %3 = icmp eq i32* %0, inttoptr (i64 2097865012304223517 to i32*)
  br i1 %3, label %"_ZN14Box$LT$u32$GT$9drop.118117h48c45852b94c565eE.exit", label %cond.i

cond.i:                                           ; preds = %unwind_custom_
  %4 = bitcast i32* %0 to i8*
  tail call void @je_sdallocx(i8* %4, i64 4, i32 0)
  br label %"_ZN14Box$LT$u32$GT$9drop.118117h48c45852b94c565eE.exit"

"_ZN14Box$LT$u32$GT$9drop.118117h48c45852b94c565eE.exit": ; preds = %unwind_custom_, %cond.i
  resume { i8*, i32 } %2
}
