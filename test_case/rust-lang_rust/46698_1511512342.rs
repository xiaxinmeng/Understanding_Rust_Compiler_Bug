llvm
; example::foo
; Function Attrs: mustprogress nofree noinline norecurse nosync nounwind nonlazybind readnone willreturn uwtable
define internal fastcc noundef i64 @_ZN7example3foo17ha5f3a2efc30301c8E(i64 noundef %x.1, i64 noundef %y.1) unnamed_addr #2 !dbg !760 {
start:
  call void @llvm.dbg.value(metadata ptr poison, metadata !766, metadata !DIExpression(DW_OP_LLVM_fragment, 0, 64)), !dbg !768
  call void @llvm.dbg.value(metadata i64 %x.1, metadata !766, metadata !DIExpression(DW_OP_LLVM_fragment, 64, 64)), !dbg !768
  call void @llvm.dbg.value(metadata ptr poison, metadata !767, metadata !DIExpression(DW_OP_LLVM_fragment, 0, 64)), !dbg !768
  call void @llvm.dbg.value(metadata i64 %y.1, metadata !767, metadata !DIExpression(DW_OP_LLVM_fragment, 64, 64)), !dbg !768
  %0 = add i64 %y.1, %x.1, !dbg !769
  ret i64 %0, !dbg !770
}
