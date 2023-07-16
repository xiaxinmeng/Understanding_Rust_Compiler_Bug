plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0599]: no method named `enter` found for struct `InferCtxtBuilder` in the current scope
    |
    |
772 |             if cx.tcx.infer_ctxt().enter(|infer_ctxt| {
    |                                    ^^^^^ method not found in `InferCtxtBuilder<'_>`
error: unused import: `InferCtxtExt`
  --> compiler/rustc_lint/src/builtin.rs:57:36
   |
   |
57 | use rustc_trait_selection::infer::{InferCtxtExt, TyCtxtInferExt};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_lint` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to 2 previous errors
