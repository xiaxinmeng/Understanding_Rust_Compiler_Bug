plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0271]: type mismatch resolving `<&[_; 0] as IntoIterator>::Item == rustc_middle::ty::GenericArg<'_>`
     |
     |
430  |     let trait_ref = ty::TraitRef { def_id: bound, substs: tcx.mk_substs_trait(ty, &[]) };
     |                                                               ---------------     ^^^ expected struct `rustc_middle::ty::GenericArg`, found reference
     |                                                               required by a bound introduced by this call
     |
     = note: expected struct `rustc_middle::ty::GenericArg<'_>`
             found reference `&_`
             found reference `&_`
note: required by a bound in `rustc_middle::ty::TyCtxt::<'tcx>::mk_substs_trait`
     |
     |
2815 |         rest: impl IntoIterator<Item = GenericArg<'tcx>>,
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `rustc_middle::ty::TyCtxt::<'tcx>::mk_substs_trait`
For more information about this error, try `rustc --explain E0271`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
