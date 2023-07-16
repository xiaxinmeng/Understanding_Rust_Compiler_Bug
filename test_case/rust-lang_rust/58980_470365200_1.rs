
define i32 @variadic(i8* %fmt, ...) unnamed_addr #3 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %0 = invoke i32 @vprintf(i8* %fmt, i64* align 8 dereferenceable(24) undef)
          to label %bb2 unwind label %cleanup
  ...
