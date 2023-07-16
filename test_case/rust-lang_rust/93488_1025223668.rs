plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `rustc_rayon::iter::IndexedParallelIterator`
   --> compiler/rustc_middle/src/hir/map/mod.rs:559:13
    |
559 |         use rustc_rayon::iter::IndexedParallelIterator;
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`

error[E0277]: `V` cannot be shared between threads safely
   --> compiler/rustc_middle/src/hir/map/mod.rs:722:17
    |
722 |                   par_for_each_in(&*module.foreign_items, |id| {
    |  _________________^^^^^^^^^^^^^^^_________________________-
    | |                 |
    | |                 `V` cannot be shared between threads safely
723 | |                     visitor.visit_foreign_item(self.foreign_item(*id))
724 | |                 })
    | |_________________- within this `[closure@compiler/rustc_middle/src/hir/map/mod.rs:722:57: 724:18]`
    = note: required because it appears within the type `&V`
    = note: required because it appears within the type `&V`
    = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:722:57: 724:18]`
note: required by a bound in `par_for_each_in`
    |
    |
340 |             for_each: impl Fn(T::Item) + Sync + Send,
    |                                          ^^^^ required by this bound in `par_for_each_in`
    |
    |
705 |         V: ParItemLikeVisitor<'hir> + std::marker::Sync,

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
