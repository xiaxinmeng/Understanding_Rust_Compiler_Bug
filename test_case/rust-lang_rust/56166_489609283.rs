rust
pub trait FnTypeExt<'tcx> {
    fn of_instance(cx: &CodegenCx<'ll, 'tcx>, instance: &ty::Instance<'tcx>) -> Self;
    fn new(cx: &CodegenCx<'ll, 'tcx>,
           sig: ty::FnSig<'tcx>,
           extra_args: &[Ty<'tcx>]) -> Self;
    fn new_vtable(cx: &CodegenCx<'ll, 'tcx>,
                  sig: ty::FnSig<'tcx>,
                  extra_args: &[Ty<'tcx>]) -> Self;
    fn new_internal(
        cx: &CodegenCx<'ll, 'tcx>,
        sig: ty::FnSig<'tcx>,
        extra_args: &[Ty<'tcx>],
        mk_arg_type: impl Fn(Ty<'tcx>, Option<usize>) -> ArgType<'tcx, Ty<'tcx>>,
    ) -> Self;
    fn adjust_for_abi(&mut self,
                      cx: &CodegenCx<'ll, 'tcx>,
                      abi: Abi);
    fn llvm_type(&self, cx: &CodegenCx<'ll, 'tcx>) -> &'ll Type;
    fn ptr_to_llvm_type(&self, cx: &CodegenCx<'ll, 'tcx>) -> &'ll Type;
    fn llvm_cconv(&self) -> llvm::CallConv;
    fn apply_attrs_llfn(&self, llfn: &'ll Value);
    fn apply_attrs_callsite(&self, bx: &mut Builder<'a, 'll, 'tcx>, callsite: &'ll Value);
}
