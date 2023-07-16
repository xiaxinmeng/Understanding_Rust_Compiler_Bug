plain
    Checking clippy_lints v0.1.67 (/checkout/src/tools/clippy/clippy_lints)
error[E0432]: unresolved import `rustc_middle::ty::PredicateKind::Trait`
 --> src/tools/clippy/clippy_lints/src/future_not_send.rs:7:45
  |
7 | use rustc_middle::ty::{EarlyBinder, Opaque, PredicateKind::Trait};
  |                                             ^^^^^^^^^^^^^^^^^^^^ no `Trait` in `ty::PredicateKind`
help: consider importing one of these items instead
  |
  |
7 | use rustc_middle::ty::{EarlyBinder, Opaque, clippy_utils::ty::ExprFnSig::Trait;
  |                                             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
7 | use rustc_middle::ty::{EarlyBinder, Opaque, crate::future_not_send::traits::specialization_graph::Node::Trait;
  |                                             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
7 | use rustc_middle::ty::{EarlyBinder, Opaque, rustc_ast::GenericBound::Trait;
  |                                             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
7 | use rustc_middle::ty::{EarlyBinder, Opaque, rustc_ast::ItemKind::Trait;
    and 13 other candidates


error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
     |
     |
1100 |             if let PredicateKind::Projection(projection_predicate) = predicate.kind().skip_binder() {
     |                                   ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
     |
     |
1114 |             if let PredicateKind::Trait(trait_predicate) = predicate.kind().skip_binder()
     |                                   ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
     |
     |
1176 |             if let PredicateKind::Trait(trait_predicate) = predicate.kind().skip_binder()
     |                                   ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    |
    |
502 |         if let PredicateKind::Trait(p) = p.kind().skip_binder()
    |                               ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    |
    |
515 |                 tcx.mk_predicate(Binder::dummy(PredicateKind::Trait(TraitPredicate {
    |                                                               ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/tools/clippy/clippy_lints/src/methods/unnecessary_to_owned.rs:344:28
344 |             PredicateKind::Trait(trait_predicate) => {
    |                            ^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
   --> src/tools/clippy/clippy_lints/src/methods/unnecessary_to_owned.rs:349:28
    |
349 |             PredicateKind::Projection(projection_predicate) => {
    |                            ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/tools/clippy/clippy_lints/src/methods/unnecessary_to_owned.rs:406:51
    |
406 | ...                   if let PredicateKind::Trait(trait_predicate) =  predicate.kind().skip_binder()
    |                                             ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:127:45
    |
127 |                     Some(ty::PredicateKind::Trait(pred)) if pred.def_id() != sized_trait => Some(pred),
    |                                             ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
    |
    |
702 |                 PredicateKind::Projection(p.with_self_ty(cx.tcx, ty)),
    |                                ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   |
   |
48 |             if let PredicateKind::Trait(poly_trait_pred) = pred.kind().skip_binder();
   |                                   ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
   |
   |
66 |         if let ty::PredicateKind::Projection(pred) = proj_pred.kind().skip_binder() {
   |                                   ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`
Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `clippy_lints` due to 13 previous errors
Build completed unsuccessfully in 0:04:24
