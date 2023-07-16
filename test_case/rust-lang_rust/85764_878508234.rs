plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `predicate_must_hold` found for mutable reference `&mut traits::select::SelectionContext<'cx, 'tcx>` in the current scope
    |
    |
207 |                 if selcx.predicate_must_hold(o_neg) {
    |                          ^^^^^^^^^^^^^^^^^^^ method not found in `&mut traits::select::SelectionContext<'cx, 'tcx>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection`
