plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: the trait bound `(): SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_borrowck/src/borrowck_errors.rs:270:40
    |
270 |         self.infcx.tcx.sess.create_err(err)
    |                             ---------- ^^^ the trait `SessionDiagnostic<'_>` is not implemented for `()`
    |                             required by a bound introduced by this call
    |
note: required by a bound in `Session::create_err`
   --> /checkout/compiler/rustc_session/src/session.rs:466:19
   --> /checkout/compiler/rustc_session/src/session.rs:466:19
    |
466 |         err: impl SessionDiagnostic<'a>,
    |                   ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::create_err`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to previous error
