plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
   --> compiler/rustc_typeck/src/check/method/suggest.rs:996:24
    |
996 |                 if let ty::PredicateKind::Trait(trait_pred, _) = pred.kind().skip_binder() {
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:479:5
    |
    |
479 |     Trait(TraitPredicate<'tcx>),
    |     --------------------------- tuple variant defined here
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
