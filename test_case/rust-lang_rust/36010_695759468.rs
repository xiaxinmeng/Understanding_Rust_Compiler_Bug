llvm-ir
define zeroext i1 @_ZN7example7foo_box17hd63365026be7dd71E(i32** nocapture align 8 dereferenceable(8) %a, i32* noalias nonnull align 4 %b) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !29 {
  %0 = ptrtoint i32* %b to i64
  %1 = bitcast i32** %a to i64*, !dbg !30
  store i64 %0, i64* %1, align 8, !dbg !43, !noalias !45
  ret i1 true, !dbg !50
}
