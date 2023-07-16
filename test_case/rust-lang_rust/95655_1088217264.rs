plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `std::slice::Iter<'_, rustc_hir::ItemId>: ParallelIterator` is not satisfied
   --> compiler/rustc_middle/src/hir/map/mod.rs:167:25
    |
167 |         par_for_each_in(self.tcx.hir_crate_items(()).items.iter(), |id| f(*id));
    |         --------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ParallelIterator` is not implemented for `std::slice::Iter<'_, rustc_hir::ItemId>`
    |         required by a bound introduced by this call
    |
    |
    = note: required because of the requirements on the impl of `IntoParallelIterator` for `std::slice::Iter<'_, rustc_hir::ItemId>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
