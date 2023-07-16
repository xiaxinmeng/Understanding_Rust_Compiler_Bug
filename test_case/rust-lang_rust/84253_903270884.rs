
@_ZN5test28cmp_same17hf172c8bcc9b7859cE = unnamed_addr alias i1 (), i1 ()* @_ZN5test27cmp_ord17ha9ee7b72078df8e5E
@_ZN5test28cmp_ord217h23f0a5da1c84ecaaE = unnamed_addr alias i1 (), i1 ()* @_ZN5test27cmp_ord17ha9ee7b72078df8e5E

; test2::cmp_diff_order
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn
define zeroext i1 @_ZN5test214cmp_diff_order17hd0c16acee48eb39bE() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  ret i1 false
}

; test2::cmp_ord
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn
define zeroext i1 @_ZN5test27cmp_ord17ha9ee7b72078df8e5E() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  ret i1 true
}
