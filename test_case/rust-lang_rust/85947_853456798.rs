plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/librustdoc/clean/mod.rs:349:13
    |
349 |             ty::PredicateKind::Trait(pred, _) => Some(bound_predicate.rebind(pred).clean(cx)),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
349 |             ty::PredicateKind::Trait(pred, _, _) => Some(bound_predicate.rebind(pred).clean(cx)),


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
    --> src/librustdoc/clean/mod.rs:1562:29
     |
1562 | ...                   ty::PredicateKind::Trait(tr, _constness) => {
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
     |
     |
470  |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
     |     -------------------------------------------------------------- tuple variant defined here
     |
help: use `_` to explicitly ignore each field
     |
1562 |                             ty::PredicateKind::Trait(tr, _constness, _) => {


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


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/librustdoc/clean/auto_trait.rs:468:25
    |
468 |                         ty::PredicateKind::Trait(pred, _) => pred.def_id() == sized_trait,
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
468 |                         ty::PredicateKind::Trait(pred, _, _) => pred.def_id() == sized_trait,


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/librustdoc/clean/simplify.rs:132:20
    |
132 |             if let ty::PredicateKind::Trait(pred, _) = pred.kind().skip_binder() {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:470:5
    |
    |
470 |     Trait(TraitPredicate<'tcx>, Constness, ImplicitTraitPredicate),
    |     -------------------------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
132 |             if let ty::PredicateKind::Trait(pred, _, _) = pred.kind().skip_binder() {

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0023`.
