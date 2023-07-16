plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `associated_items_for_impl_trait_in_trait` found for struct `context::TyCtxt<'tcx>` in the current scope
    --> compiler/rustc_middle/src/ty/mod.rs:2582:26
     |
2582 |             return !self.associated_items_for_impl_trait_in_trait(trait_item_def_id).is_empty();
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `is_impl_trait_in_trait`
    ::: compiler/rustc_middle/src/ty/context.rs:478:1
     |
478  | pub struct TyCtxt<'tcx> {
478  | pub struct TyCtxt<'tcx> {
     | ----------------------- method `associated_items_for_impl_trait_in_trait` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
