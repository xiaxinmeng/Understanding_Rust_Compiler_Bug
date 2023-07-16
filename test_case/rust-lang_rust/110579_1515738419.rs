plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0277]: the trait bound `DiagnosticMessage: From<&std::string::String>` is not satisfied
   --> compiler/rustc_passes/src/hir_id_validator.rs:34:59
    |
34  |             tcx.sess.delay_span_bug(rustc_span::DUMMY_SP, &message);
    |                      --------------                       ^^^^^^^^ the trait `From<&std::string::String>` is not implemented for `DiagnosticMessage`
    |                      required by a bound introduced by this call
    |
    |
    = note: required for `&std::string::String` to implement `Into<DiagnosticMessage>`
note: required by a bound in `Session::delay_span_bug`
    |
    |
670 |         msg: impl Into<DiagnosticMessage>,
    |                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::delay_span_bug`
    |
    |
34  |             tcx.sess.delay_span_bug(rustc_span::DUMMY_SP, &*message);

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
