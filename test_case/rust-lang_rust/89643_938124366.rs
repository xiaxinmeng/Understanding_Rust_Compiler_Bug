plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: no rules expected the token `}`
   --> compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs:162:46
162 |                         pub struct RegionId {}
162 |                         pub struct RegionId {}
    |                                              ^ no rules expected this token in macro call

error[E0412]: cannot find type `IndexVec` in this scope
   --> compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs:168:48
    |
168 |                     let mut connected_regions: IndexVec<RegionId, _> = Default::default();
    |
help: consider importing this struct
    |
    |
1   | use rustc_index::vec::IndexVec;

error[E0412]: cannot find type `RegionId` in this scope
   --> compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs:168:57
    |
    |
117 | impl ItemLikeVisitor<'v> for InherentOverlapChecker<'tcx> {
    |     - help: you might be missing a type parameter: `<RegionId>`
...
168 |                     let mut connected_regions: IndexVec<RegionId, _> = Default::default();

For more information about this error, try `rustc --explain E0412`.
error: could not compile `rustc_typeck` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
