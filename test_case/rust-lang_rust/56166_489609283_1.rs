rust
fn llvm_type(&self, cx: &CodegenCx<'ll, 'tcx>) -> &'ll Type;
fn ptr_to_llvm_type(&self, cx: &CodegenCx<'ll, 'tcx>) -> &'ll Type;
fn llvm_cconv(&self) -> llvm::CallConv;
fn apply_attrs_llfn(&self, llfn: &'ll Value);
