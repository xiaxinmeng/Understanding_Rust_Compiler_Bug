plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0277]: the trait bound `impl Iterator<Item = ItemId>: ParallelIterator` is not satisfied
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1864:21
     |
1864 |     par_for_each_in(items.items(), |item| tcx.ensure().check_well_formed(item.def_id));
     |     --------------- ^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `impl Iterator<Item = ItemId>`
     |     required by a bound introduced by this call
     |
     = help: the following other types implement trait `ParallelIterator`:
     = help: the following other types implement trait `ParallelIterator`:
               either::Either<L, R>
               indexmap::rustc::map::IntoParIter<K, V>
               indexmap::rustc::map::ParIter<'a, K, V>
               indexmap::rustc::map::ParIterMut<'a, K, V>
               indexmap::rustc::set::IntoParIter<T>
               indexmap::rustc::set::ParIter<'a, T>
               rustc_rayon::array::IntoIter<T, N>
               rustc_rayon::collections::binary_heap::Drain<'a, T>
             and 110 others
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `impl Iterator<Item = ItemId>`

error[E0277]: the trait bound `impl Iterator<Item = ImplItemId>: ParallelIterator` is not satisfied
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1865:21
     |
1865 |     par_for_each_in(items.impl_items(), |item| tcx.ensure().check_well_formed(item.def_id));
     |     --------------- ^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `impl Iterator<Item = ImplItemId>`
     |     required by a bound introduced by this call
     |
     = help: the following other types implement trait `ParallelIterator`:
     = help: the following other types implement trait `ParallelIterator`:
               either::Either<L, R>
               indexmap::rustc::map::IntoParIter<K, V>
               indexmap::rustc::map::ParIter<'a, K, V>
               indexmap::rustc::map::ParIterMut<'a, K, V>
               indexmap::rustc::set::IntoParIter<T>
               indexmap::rustc::set::ParIter<'a, T>
               rustc_rayon::array::IntoIter<T, N>
               rustc_rayon::collections::binary_heap::Drain<'a, T>
             and 110 others
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `impl Iterator<Item = ImplItemId>`

error[E0277]: the trait bound `impl Iterator<Item = TraitItemId>: ParallelIterator` is not satisfied
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1866:21
     |
1866 |     par_for_each_in(items.trait_items(), |item| tcx.ensure().check_well_formed(item.def_id));
     |     --------------- ^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `impl Iterator<Item = TraitItemId>`
     |     required by a bound introduced by this call
     |
     = help: the following other types implement trait `ParallelIterator`:
     = help: the following other types implement trait `ParallelIterator`:
               either::Either<L, R>
               indexmap::rustc::map::IntoParIter<K, V>
               indexmap::rustc::map::ParIter<'a, K, V>
               indexmap::rustc::map::ParIterMut<'a, K, V>
               indexmap::rustc::set::IntoParIter<T>
               indexmap::rustc::set::ParIter<'a, T>
               rustc_rayon::array::IntoIter<T, N>
               rustc_rayon::collections::binary_heap::Drain<'a, T>
             and 110 others
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `impl Iterator<Item = TraitItemId>`

error[E0277]: the trait bound `impl Iterator<Item = ForeignItemId>: ParallelIterator` is not satisfied
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1867:21
     |
1867 |     par_for_each_in(items.foreign_items(), |item| tcx.ensure().check_well_formed(item.def_id));
     |     --------------- ^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `impl Iterator<Item = ForeignItemId>`
     |     required by a bound introduced by this call
     |
     = help: the following other types implement trait `ParallelIterator`:
     = help: the following other types implement trait `ParallelIterator`:
               either::Either<L, R>
               indexmap::rustc::map::IntoParIter<K, V>
               indexmap::rustc::map::ParIter<'a, K, V>
               indexmap::rustc::map::ParIterMut<'a, K, V>
               indexmap::rustc::set::IntoParIter<T>
               indexmap::rustc::set::ParIter<'a, T>
               rustc_rayon::array::IntoIter<T, N>
               rustc_rayon::collections::binary_heap::Drain<'a, T>
             and 110 others
     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `impl Iterator<Item = ForeignItemId>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to 4 previous errors
