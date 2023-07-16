plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/auto_trait.rs:324:32
    |
324 |             ty::PredicateKind::Trait(poly_trait_pred) => {
    |                                ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/auto_trait.rs:327:32
    |
327 |             ty::PredicateKind::Projection(poly_proj_pred) => {
    |                                ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/auto_trait.rs:454:44
    |
454 |                         ty::PredicateKind::Trait(pred) => pred.def_id() == sized_trait,
    |                                            ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/simplify.rs:135:39
    |
135 |             if let ty::PredicateKind::Trait(pred) = pred.kind().skip_binder() {
    |                                       ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:305:28
    |
305 |         ty::PredicateKind::Trait(pred) => {
    |                            ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `RegionOutlives` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:308:28
    |
308 |         ty::PredicateKind::RegionOutlives(pred) => clean_region_outlives_predicate(pred),
    |                            ^^^^^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `TypeOutlives` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:309:28
    |
309 |         ty::PredicateKind::TypeOutlives(pred) => clean_type_outlives_predicate(pred, cx),
    |                            ^^^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:310:28
    |
310 |         ty::PredicateKind::Projection(pred) => {
    |                            ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:692:40
    |
692 |                     ty::PredicateKind::Trait(pred) => {
    |                                        ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `TypeOutlives` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:697:40
    |
697 |                     ty::PredicateKind::TypeOutlives(ty::OutlivesPredicate(ty, _reg)) => {
    |                                        ^^^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
   --> src/librustdoc/clean/mod.rs:702:40
702 |                     ty::PredicateKind::Projection(p) => {
    |                                        ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    --> src/librustdoc/clean/mod.rs:1775:36
     |
1775 |                 ty::PredicateKind::Trait(tr) => bound_predicate.rebind(tr.trait_ref),
     |                                    ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `TypeOutlives` found for enum `PredicateKind` in the current scope
    --> src/librustdoc/clean/mod.rs:1776:36
     |
1776 |                 ty::PredicateKind::TypeOutlives(ty::OutlivesPredicate(_ty, reg)) => {
     |                                    ^^^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
    --> src/librustdoc/clean/mod.rs:1795:47
     |
1795 |                     if let ty::PredicateKind::Projection(proj) = bound.kind().skip_binder() {
     |                                               ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1812:57
     |
     |
1812 |             Some(clean_poly_trait_ref_with_bindings(cx, trait_ref, bindings))
     |                  ----------------------------------     ^^^^^^^^^ expected struct `rustc_middle::ty::TraitRef`, found struct `TraitPredicate`
     |                  arguments to this function are incorrect
     |
     = note: expected struct `Binder<'_, rustc_middle::ty::TraitRef<'_>>`
     = note: expected struct `Binder<'_, rustc_middle::ty::TraitRef<'_>>`
                found struct `Binder<'_, TraitPredicate<'_>>`
    --> src/librustdoc/clean/mod.rs:174:4
     |
174  | fn clean_poly_trait_ref_with_bindings<'tcx>(
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
175  |     cx: &mut DocContext<'tcx>,
176  |     poly_trait_ref: ty::PolyTraitRef<'tcx>,

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 15 previous errors
