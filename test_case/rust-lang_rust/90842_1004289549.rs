plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0422]: cannot find struct, variant or union type `ImplVisitor` in this scope
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1809:27
     |
1809 |         let mut visitor = ImplVisitor { tcx, impls: FxIndexMap::default() };
     |                           ^^^^^^^^^^^ help: a struct with a similar name exists: `ImplsVisitor`
2054 | struct ImplsVisitor<'tcx> {
2054 | struct ImplsVisitor<'tcx> {
     | ------------------------- similarly named struct `ImplsVisitor` defined here

error[E0277]: the trait bound `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>: ParallelIterator` is not satisfied
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:2098:14
     |
2098 |     par_iter(tcx.mir_keys(())).for_each(|&def_id| {
     |     -------- ^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>`
     |     required by a bound introduced by this call
     |
     |
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>`
note: required by a bound in `par_iter`
     |
     |
334  |         pub fn par_iter<T: IntoParallelIterator>(t: T) -> T::Iter {
     |                            ^^^^^^^^^^^^^^^^^^^^ required by this bound in `par_iter`

error[E0599]: the method `for_each` exists for reference `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:2098:32
     |
2098 |     par_iter(tcx.mir_keys(())).for_each(|&def_id| {
     |                                ^^^^^^^^ method cannot be called on `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.7.0/src/set.rs:63:1
     |
     |
63   | pub struct IndexSet<T, S = RandomState> {
     | --------------------------------------- doesn't satisfy `_: Iterator`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>: Iterator`
             which is required by `&mut &indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>: Iterator`
             `indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>: Iterator`
             which is required by `&mut indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>: Iterator`

error[E0277]: the trait bound `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>: ParallelIterator` is not satisfied
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:2098:5
     |
2098 |     par_iter(tcx.mir_keys(())).for_each(|&def_id| {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>`
     |
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `&indexmap::set::IndexSet<LocalDefId, BuildHasherDefault<FxHasher>>`
error: unused import: `ParallelIterator`
 --> compiler/rustc_metadata/src/rmeta/encoder.rs:7:56
  |
  |
7 | use rustc_data_structures::sync::{join, par_iter, Lrc, ParallelIterator};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
Some errors have detailed explanations: E0277, E0422, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_metadata` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
