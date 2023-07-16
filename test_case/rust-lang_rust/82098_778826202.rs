llvm
define void @_ZN7example12map_ints_new17hd42cfbe0c1377431E([8 x float]* noalias nocapture sret dereferenceable(32) %0, [8 x i32]* noalias nocapture readonly dereferenceable(32) %s) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !166 {
start:
  %1 = bitcast [8 x i32]* %s to <8 x i32>*, !dbg !167
  %2 = load <8 x i32>, <8 x i32>* %1, align 4, !dbg !167
  %3 = uitofp <8 x i32> %2 to <8 x float>, !dbg !168
  %4 = bitcast [8 x float]* %0 to <8 x float>*, !dbg !180
  store <8 x float> %3, <8 x float>* %4, align 4, !dbg !180, !alias.scope !181, !noalias !184
  ret void, !dbg !186
}
