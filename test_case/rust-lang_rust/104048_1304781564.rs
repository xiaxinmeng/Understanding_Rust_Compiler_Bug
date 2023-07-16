plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no method named `position` found for reference `&'tcx rustc_hir::Lifetime` in the current scope
     |
     |
1344 | ...                   let lifetime_sugg = match lifetime_ref.position() {
     |                                                              ^^^^^^^^ method not found in `&'tcx rustc_hir::Lifetime`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_hir_analysis` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_analysis` due to previous error
