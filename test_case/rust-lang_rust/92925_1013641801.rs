plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs:620:43
    |
611 |               let ty = match self.tcx.asyncness(fn_id.owner) {
    |                        ------------------------------------- `match` arms have incompatible types
612 |                   hir::IsAsync::Async => self.tcx.infer_ctxt().enter(|infcx| {
    |  ________________________________________-
613 | |                     infcx.get_impl_future_output_ty(ty).unwrap_or_else(|| {
614 | |                         span_bug!(
615 | |                             fn_decl.output.span(),
618 | |                     })
619 | |                 }),
619 | |                 }),
    | |__________________- this is found to be of type `Binder<'_, &TyS<'_>>`
620 |                   hir::IsAsync::NotAsync => ty,
    |                                             ^^ expected struct `Binder`, found `&TyS<'_>`
    |
    = note:   expected type `Binder<'_, &TyS<'_>>`
            found reference `&TyS<'_>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
