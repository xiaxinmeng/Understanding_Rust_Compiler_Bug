plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
   --> compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs:795:45
    |
795 |     fn note_synth_provided(&self, err: &mut DiagnosticBuilder<'_>) {
    |                                             ^^^^^^^^^^^^^^^^^ expected 1 generic argument
    |
note: struct defined here, with 1 generic parameter: `G`
    |
    |
19  | pub struct DiagnosticBuilder<'a, G: EmissionGuarantee> {
help: add missing generic argument
    |
    |
795 |     fn note_synth_provided(&self, err: &mut DiagnosticBuilder<'_, G>) {

For more information about this error, try `rustc --explain E0107`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
