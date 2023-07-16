plain
    Checking semver v0.11.0
    Checking clippy_utils v0.1.54 (/checkout/src/tools/clippy/clippy_utils)
    Checking toml v0.5.7
    Checking url v2.2.2
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/tools/clippy/clippy_utils/src/ty.rs:158:24
    |
158 |                 if let ty::PredicateKind::Trait(trait_predicate, _) = predicate.kind().skip_binder() {
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
158 |                 if let ty::PredicateKind::Trait(trait_predicate, _, _) = predicate.kind().skip_binder() {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
