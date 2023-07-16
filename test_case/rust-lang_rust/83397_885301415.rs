plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0277]: the trait bound `HirId: rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not satisfied
    |
    |
610 |         let stab = self.tcx.lookup_stability(hir_id);
    |                                              ^^^^^^ the trait `rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not implemented for `HirId`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_passes`
