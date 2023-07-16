plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0422]: cannot find struct, variant or union type `LifetimeOutliveErr` in this scope
    |
    |
680 |         let err = LifetimeOutliveErr { span: *span };
    |                   ^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `LifeTimeOutLiveErr`
   ::: compiler/rustc_borrowck/src/session_diagnostics.rs:113:1
    |
    |
113 | pub(crate) struct LifeTimeOutLiveErr {
    | ------------------------------------ similarly named struct `LifeTimeOutLiveErr` defined here

error: unused import: `LifeTimeOutLiveErr`
   |
   |
29 |     FnMutError, FnMutReturnTypeErr, GenericDoesNotLiveLongEnough, LifeTimeOutLiveErr,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0422`.
error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 2 previous errors
