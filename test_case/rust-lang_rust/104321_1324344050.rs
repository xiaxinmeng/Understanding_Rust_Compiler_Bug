plain
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
[RUSTC-TIMING] rustc_parse test:false 26.883
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
[RUSTC-TIMING] rustc_save_analysis test:false 10.129
error[E0271]: type mismatch resolving `<&[_; 0] as IntoIterator>::Item == rustc_middle::ty::GenericArg<'_>`
    |
    |
329 |         ty::TraitRef { def_id: fn_trait_def_id, substs: tcx.mk_substs_trait(self_ty, &[]) };
    |                                                             ---------------          ^^^ expected struct `rustc_middle::ty::GenericArg`, found reference
    |                                                             required by a bound introduced by this call
    |
    = note: expected struct `rustc_middle::ty::GenericArg<'_>`
            found reference `&_`
            found reference `&_`
note: required by a bound in `rustc_middle::ty::TyCtxt::<'tcx>::mk_substs_trait`
For more information about this error, try `rustc --explain E0271`.
[RUSTC-TIMING] rustc_trait_selection test:false 2.615
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
