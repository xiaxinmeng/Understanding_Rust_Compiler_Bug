plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `std::iter::Copied<std::slice::Iter<'_, rustc_hir::ItemId>>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:50:28
   |
50 |     pub fn items(&self) -> impl Iterator<Item = ItemId> + ParallelIterator<Item = ItemId> + '_ {
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::iter::Copied<std::slice::Iter<'_, rustc_hir::ItemId>>`
   = help: the following other types implement trait `ParallelIterator`:
   = help: the following other types implement trait `ParallelIterator`:
             Either<L, R>
             FlatMapIter<I, F>
             FlattenIter<I>
             FoldWith<I, U, F>
             Interleave<I, J>
             InterleaveShortest<I, J>
             IterBridge<Iter>
             MapInit<I, INIT, F>


error[E0277]: the trait bound `std::iter::Copied<std::slice::Iter<'_, rustc_hir::TraitItemId>>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:56:10
   |
56 |     ) -> impl Iterator<Item = TraitItemId> + ParallelIterator<Item = TraitItemId> + '_ {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::iter::Copied<std::slice::Iter<'_, rustc_hir::TraitItemId>>`
   = help: the following other types implement trait `ParallelIterator`:
   = help: the following other types implement trait `ParallelIterator`:
             Either<L, R>
             FlatMapIter<I, F>
             FlattenIter<I>
             FoldWith<I, U, F>
             Interleave<I, J>
             InterleaveShortest<I, J>
             IterBridge<Iter>
             MapInit<I, INIT, F>


error[E0277]: the trait bound `std::iter::Copied<std::slice::Iter<'_, rustc_hir::ImplItemId>>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:62:10
   |
62 |     ) -> impl Iterator<Item = ImplItemId> + ParallelIterator<Item = ImplItemId> + '_ {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::iter::Copied<std::slice::Iter<'_, rustc_hir::ImplItemId>>`
   = help: the following other types implement trait `ParallelIterator`:
   = help: the following other types implement trait `ParallelIterator`:
             Either<L, R>
             FlatMapIter<I, F>
             FlattenIter<I>
             FoldWith<I, U, F>
             Interleave<I, J>
             InterleaveShortest<I, J>
             IterBridge<Iter>
             MapInit<I, INIT, F>


error[E0277]: the trait bound `std::iter::Copied<std::slice::Iter<'_, rustc_hir::ForeignItemId>>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:68:10
   |
68 |     ) -> impl Iterator<Item = ForeignItemId> + ParallelIterator<Item = ForeignItemId> + '_ {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::iter::Copied<std::slice::Iter<'_, rustc_hir::ForeignItemId>>`
   = help: the following other types implement trait `ParallelIterator`:
   = help: the following other types implement trait `ParallelIterator`:
             Either<L, R>
             FlatMapIter<I, F>
             FlattenIter<I>
             FoldWith<I, U, F>
             Interleave<I, J>
             InterleaveShortest<I, J>
             IterBridge<Iter>
             MapInit<I, INIT, F>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
