plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0599]: no variant named `Lasts` found for enum `MustValidFor<'_>`
    |
    |
309 |                     let sub_label = MustValidFor::Lasts {
    |                                                   ^^^^^ variant not found in `MustValidFor<'_>`
   ::: compiler/rustc_borrowck/src/session_diagnostics.rs:315:1
    |
    |
315 | pub(crate) enum MustValidFor<'a> {
    | -------------------------------- variant `Lasts` not found here
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to previous error
