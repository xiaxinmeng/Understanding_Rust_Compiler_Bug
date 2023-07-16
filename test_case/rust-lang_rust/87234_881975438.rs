plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `std::collections::btree_map::Keys<'_, rustc_hir::BodyId, rustc_hir::Body<'_>>: ParallelIterator` is not satisfied
    --> compiler/rustc_middle/src/ty/mod.rs:1636:18
     |
1636 |         par_iter(self.hir().krate().bodies.keys())
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::collections::btree_map::Keys<'_, rustc_hir::BodyId, rustc_hir::Body<'_>>`
    ::: /checkout/compiler/rustc_data_structures/src/sync.rs:334:28
     |
     |
334  |         pub fn par_iter<T: IntoParallelIterator>(t: T) -> T::Iter {
     |                            -------------------- required by this bound in `par_iter`
     |
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `std::collections::btree_map::Keys<'_, rustc_hir::BodyId, rustc_hir::Body<'_>>`

error: unused import: `ParallelIterator`
  --> compiler/rustc_middle/src/ty/mod.rs:36:51
   |
36 | use rustc_data_structures::sync::{self, par_iter, ParallelIterator};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`
