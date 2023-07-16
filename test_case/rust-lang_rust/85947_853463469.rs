plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `tr` in this scope
    --> src/librustdoc/clean/mod.rs:1563:56
     |
1563 | ...                   bound_predicate.rebind(tr.trait_ref)
     |                                              ^^ help: a local variable with a similar name exists: `ty`

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/librustdoc/clean/mod.rs:629:25
    |
629 |                         ty::PredicateKind::Trait(pred, _constness) => {
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
629 |                         ty::PredicateKind::Trait(pred, _constness, _) => {


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/librustdoc/clean/auto_trait.rs:319:13
    |
319 |             ty::PredicateKind::Trait(poly_trait_pred, _) => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
319 |             ty::PredicateKind::Trait(poly_trait_pred, _, _) => {

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0023, E0425.
