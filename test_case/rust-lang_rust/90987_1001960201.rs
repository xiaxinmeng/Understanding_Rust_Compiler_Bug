plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0603]: trait `InferCtxtPrivExt` is private
    --> compiler/rustc_trait_selection/src/traits/fulfill.rs:30:37
     |
30   | use crate::traits::error_reporting::InferCtxtPrivExt;
     |                                     ^^^^^^^^^^^^^^^^ private trait
     |
note: the trait `InferCtxtPrivExt` is defined here
     |
     |
1096 | trait InferCtxtPrivExt<'hir, 'tcx> {

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `try_create_suggestion_for_mismatched_const` found for reference `&rustc_infer::infer::InferCtxt<'_, '_>` in the current scope
   --> compiler/rustc_trait_selection/src/traits/fulfill.rs:644:38
    |
644 | ...                   .try_create_suggestion_for_mismatched_const(expected_found)
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&rustc_infer::infer::InferCtxt<'_, '_>`
Some errors have detailed explanations: E0599, E0603.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
