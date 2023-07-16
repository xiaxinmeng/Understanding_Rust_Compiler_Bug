
define void @_ZN5test29test_loop17he2725c5c3163608aE() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  tail call void @f()
  tail call void @f()
  tail call void @f()
  tail call void @f()
  tail call void @f()
  tail call void @f()
  ret void
}
