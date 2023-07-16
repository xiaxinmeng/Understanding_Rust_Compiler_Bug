plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0609]: no field `def_id` on type `for<'r> fn(TyCtxt<'r>, DefId) -> Option<rustc_middle::ty::TraitRef<'r>> {collect::impl_trait_ref}`
     |
     |
1228 |             .get_attrs(impl_trait_ref.def_id)


error[E0609]: no field `sees` on type `TyCtxt<'_>`
     |
1234 |                     tcx.sees
     |                         ^^^^ unknown field


error[E0609]: no field `sees` on type `TyCtxt<'_>`
     |
     |
1247 |                         .collect::<Result<Box<[_]>, _>>().map_err(|span| tcx.sees

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_typeck` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
