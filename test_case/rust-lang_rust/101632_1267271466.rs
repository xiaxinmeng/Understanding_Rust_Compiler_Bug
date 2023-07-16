plain
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0599]: no method named `enter` found for struct `InferCtxtBuilder` in the current scope
   |
   |
63 |         cx.tcx.infer_ctxt().enter(|ref infcx| {
   |                             ^^^^^ method not found in `InferCtxtBuilder<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to previous error
