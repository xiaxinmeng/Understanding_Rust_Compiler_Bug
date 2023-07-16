plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    |
    |
239 |                     ty::PredicateKind::ConstEvaluatable(def_a, substs_a),
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:486:5
    |
    |
486 |     ConstEvaluatable(ty::Unevaluated<'tcx, ()>),
    |     ------------------------------------------- tuple variant defined here

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    |
    |
240 |                     ty::PredicateKind::ConstEvaluatable(def_b, substs_b),
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:486:5
    |
    |
486 |     ConstEvaluatable(ty::Unevaluated<'tcx, ()>),
    |     ------------------------------------------- tuple variant defined here
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
