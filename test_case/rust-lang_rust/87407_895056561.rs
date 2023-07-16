plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    |
    |
239 |                     ty::PredicateKind::ConstEvaluatable(def_a, substs_a),
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 2

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    |
    |
240 |                     ty::PredicateKind::ConstEvaluatable(def_b, substs_b),
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 2
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
