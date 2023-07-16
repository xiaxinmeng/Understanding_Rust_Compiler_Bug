plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: the method `for_each` exists for struct `rustc_rayon::collections::btree_set::Iter<'_, rustc_span::def_id::LocalDefId>`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/hir/map/mod.rs:580:41
    |
580 |             par_iter(&items.submodules).for_each(|&sm| par_iter_submodules(tcx, sm, f));
    |                                         ^^^^^^^^ method cannot be called on `rustc_rayon::collections::btree_set::Iter<'_, rustc_span::def_id::LocalDefId>` due to unsatisfied trait bounds
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.3.1/src/collections/btree_set.rs:30:1
    |
    |
30  | pub struct Iter<'a, T: Ord + Sync> {
    | ---------------------------------- doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `rustc_rayon::collections::btree_set::Iter<'_, rustc_span::def_id::LocalDefId>: Iterator`
            which is required by `&mut rustc_rayon::collections::btree_set::Iter<'_, rustc_span::def_id::LocalDefId>: Iterator`
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::rustc_data_structures::sync::ParallelIterator;`

For more information about this error, try `rustc --explain E0599`.
