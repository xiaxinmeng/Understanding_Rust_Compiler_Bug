plain
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver_impl v0.0.0 (/checkout/compiler/rustc_driver_impl)
error: unused return value of `early_error_no_abort` that must be used
     |
     |
1253 |                 early_error_no_abort(ErrorOutputType::default(), &msg);
     |
     = note: ErrorGuaranteed does nothing unless returned from build_session_options
     = note: ErrorGuaranteed does nothing unless returned from build_session_options
     = note: `-D unused-must-use` implied by `-D warnings`
     |
     |
1253 |                 let _ = early_error_no_abort(ErrorOutputType::default(), &msg);

error: could not compile `rustc_driver_impl` (lib test) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_driver_impl` (lib) due to previous error
