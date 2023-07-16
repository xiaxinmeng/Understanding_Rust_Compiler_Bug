
define i64 @_ZN10playground13new_optimized17hf103a5e804c97ad6E(i64 %c) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = shl i64 %c, 3

  // This computes the alignment result for Layout::repeat.
  // It checks if the multiplication overflows and returns either 0 (LayoutErr) or 8 (self.align).
  %1 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %c, i64 16) #7
  %2 = extractvalue { i64, i1 } %1, 1
  %spec.select.i15.i = select i1 %2, i64 0, i64 8

  // This calculates the offset and Layout::padding_for in Layout::extend.
  // The codegen bug is here: At this point, the alignment can *only* be 8, not 0
  // since Err(LayoutErr) would have returned early before this point (due to ?).
  // But LLVM fails to constant-propagate this for some reason.
  %3 = add i64 %0, -1
  %4 = add i64 %3, %spec.select.i15.i
  %5 = sub nsw i64 0, %spec.select.i15.i
  %6 = and i64 %4, %5
  ret i64 %6
}
