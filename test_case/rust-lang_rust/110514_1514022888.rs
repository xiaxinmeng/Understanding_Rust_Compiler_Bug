plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused import: `fast_reject::TreatProjections`
  --> src/librustdoc/passes/collect_intra_doc_links.rs:16:24
   |
16 | use rustc_middle::ty::{fast_reject::TreatProjections, Ty, TyCtxt};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: mismatched types
   --> src/librustdoc/passes/collect_intra_doc_links.rs:778:36
    |
    |
778 |         tcx.for_each_relevant_impl(trait_, ty, |impl_| {
    |             ---------------------- ^^^^^^ expected `DefId`, found `&DefId`
    |             arguments to this method are incorrect
    |
note: method defined here
   --> /checkout/compiler/rustc_middle/src/ty/trait_def.rs:121:12
   --> /checkout/compiler/rustc_middle/src/ty/trait_def.rs:121:12
    |
121 |     pub fn for_each_relevant_impl(
    |            ^^^^^^^^^^^^^^^^^^^^^^
help: consider dereferencing the borrow
    |
778 |         tcx.for_each_relevant_impl(*trait_, ty, |impl_| {

error[E0308]: mismatched types
   --> src/librustdoc/passes/collect_intra_doc_links.rs:808:5
    |
    |
773 | ) -> FxHashSet<(DefId, DefId)> {
    |      ------------------------- expected `HashSet<(rustc_span::def_id::DefId, rustc_span::def_id::DefId), BuildHasherDefault<FxHasher>>` because of return type
808 |     impls
808 |     impls
    |     ^^^^^ expected `HashSet<(DefId, DefId), ...>`, found `Vec<(DefId, &DefId)>`
    |
    = note: expected struct `HashSet<(rustc_span::def_id::DefId, rustc_span::def_id::DefId), BuildHasherDefault<FxHasher>>`
               found struct `Vec<(rustc_span::def_id::DefId, &rustc_span::def_id::DefId)>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:01:17
