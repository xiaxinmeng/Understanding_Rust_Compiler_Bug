plain
    Checking toml v0.5.7
    Checking url v2.2.2
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.54 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    |
    |
97  | ...                   if let Trait(trait_pred, _) = obligation.predicate.kind().skip_binder() {
    |                              ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
97  |                                     if let Trait(trait_pred, _, _) = obligation.predicate.kind().skip_binder() {


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    |
    |
46  |             if let PredicateKind::Trait(poly_trait_pred, _) = pred.kind().skip_binder();
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
46  |             if let PredicateKind::Trait(poly_trait_pred, _, _) = pred.kind().skip_binder();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0023`.
