plain
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error[E0308]: mismatched types
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:2658:53
     |
2658 |                 EarlyBinder(self.normalize_ty(span, tcx.at(span).bound_type_of(def_id)))
     |                                  ------------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::Ty`, found struct `EarlyBinder`
     |                                  arguments to this function are incorrect
     |
     = note: expected struct `rustc_middle::ty::Ty<'tcx>`
     = note: expected struct `rustc_middle::ty::Ty<'tcx>`
                found struct `EarlyBinder<rustc_middle::ty::Ty<'_>>`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:111:8
     |
     |
111  |     fn normalize_ty(&self, span: Span, ty: Ty<'tcx>) -> Ty<'tcx>;

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_hir_analysis test:false 2.257
error: could not compile `rustc_hir_analysis` due to previous error
