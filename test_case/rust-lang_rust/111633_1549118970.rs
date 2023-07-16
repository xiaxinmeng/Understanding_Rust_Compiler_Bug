plain
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver_impl v0.0.0 (/checkout/compiler/rustc_driver_impl)
error[E0277]: the trait bound `DiagnosticMessage: From<&&std::string::String>` is not satisfied
     |
     |
1253 |                 early_error_no_abort(ErrorOutputType::default(), &msg);
     |                 --------------------                             ^^^^ the trait `From<&&std::string::String>` is not implemented for `DiagnosticMessage`
     |                 required by a bound introduced by this call
     |
     |
     = note: required for `&&std::string::String` to implement `Into<DiagnosticMessage>`
note: required by a bound in `early_error_no_abort`
     |
     |
1732 |     msg: impl Into<DiagnosticMessage>,
     |               ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `early_error_no_abort`
     |
     |
1253 |                 early_error_no_abort(ErrorOutputType::default(), &**msg);

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_driver_impl` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
