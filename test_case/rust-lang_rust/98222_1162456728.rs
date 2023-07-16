plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `std::slice::Iter<'_, rustc_hir::ItemId>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:67:25
   |
67 |         par_for_each_in(self.items.iter(), |&id| f(id))
   |         --------------- ^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::slice::Iter<'_, rustc_hir::ItemId>`
   |         required by a bound introduced by this call
   |
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
           and 110 others
   = note: required because of the requirements on the impl of `IntoParallelIterator` for `std::slice::Iter<'_, rustc_hir::ItemId>`

error[E0277]: the trait bound `std::slice::Iter<'_, rustc_hir::TraitItemId>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:71:25
   |
71 |         par_for_each_in(self.trait_items.iter(), |&id| f(id))
   |         --------------- ^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::slice::Iter<'_, rustc_hir::TraitItemId>`
   |         required by a bound introduced by this call
   |
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
           and 110 others
   = note: required because of the requirements on the impl of `IntoParallelIterator` for `std::slice::Iter<'_, rustc_hir::TraitItemId>`

error[E0277]: the trait bound `std::slice::Iter<'_, rustc_hir::ImplItemId>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:75:25
   |
75 |         par_for_each_in(self.impl_items.iter(), |&id| f(id))
   |         --------------- ^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::slice::Iter<'_, rustc_hir::ImplItemId>`
   |         required by a bound introduced by this call
   |
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
           and 110 others
   = note: required because of the requirements on the impl of `IntoParallelIterator` for `std::slice::Iter<'_, rustc_hir::ImplItemId>`

error[E0277]: the trait bound `std::slice::Iter<'_, rustc_hir::ForeignItemId>: ParallelIterator` is not satisfied
  --> compiler/rustc_middle/src/hir/mod.rs:79:25
   |
79 |         par_for_each_in(self.foreign_items.iter(), |&id| f(id))
   |         --------------- ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::slice::Iter<'_, rustc_hir::ForeignItemId>`
   |         required by a bound introduced by this call
   |
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
           and 110 others
   = note: required because of the requirements on the impl of `IntoParallelIterator` for `std::slice::Iter<'_, rustc_hir::ForeignItemId>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 4 previous errors
