plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0119]: conflicting implementations of trait `infer::at::ToTrace<'_>` for type `rustc_middle::ty::Effect<'_>`
   --> compiler/rustc_infer/src/infer/at.rs:391:1
    |
349 | impl<'tcx> ToTrace<'tcx> for ty::Effect<'tcx> {
...
...
391 | impl<'tcx> ToTrace<'tcx> for ty::Effect<'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `rustc_middle::ty::Effect<'_>`
For more information about this error, try `rustc --explain E0119`.
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_infer` due to previous error
