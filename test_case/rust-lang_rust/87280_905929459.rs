plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0046]: not all trait items implemented, missing: `tcx_for_anon_const_substs`
   --> compiler/rustc_trait_selection/src/traits/query/normalize.rs:115:1
    |
115 | impl<'tcx> TypeVisitor<'tcx> for MaxEscapingBoundVarVisitor {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `tcx_for_anon_const_substs` in implementation
    |
    = help: implement the missing item: `fn tcx_for_anon_const_substs(&self) -> Option<rustc_middle::ty::TyCtxt<'tcx>> { todo!() }`
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
