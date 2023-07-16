plain
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error[E0432]: unresolved import `rustc_macros::SessionDiagnostic`
 --> compiler/rustc_middle/src/error.rs:1:32
  |
1 | use rustc_macros::{Diagnostic, SessionDiagnostic};
  |                                ^^^^^^^^^^^^^^^^^ no `SessionDiagnostic` in the root

error: cannot determine resolution for the derive macro `SessionDiagnostic`
   |
52 | #[derive(SessionDiagnostic)]
   |          ^^^^^^^^^^^^^^^^^
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot find attribute `diag` in this scope
   |
   |
53 | #[diag(middle::const_eval_non_int)]

error: cannot find attribute `primary_span` in this scope
  --> compiler/rustc_middle/src/error.rs:55:7
   |
   |
55 |     #[primary_span]

    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/ty/adt.rs:461:76
    |
461 |                     tcx.sess.emit_err(crate::error::ConstEvalNonIntError { span });
    |                                                                            ^^^^ expected struct `rustc_span::Span`, found struct `TyCtxtAt`

error[E0277]: the trait bound `ConstEvalNonIntError: IntoDiagnostic<'_>` is not satisfied
   --> compiler/rustc_middle/src/ty/adt.rs:461:39
    |
461 |                     tcx.sess.emit_err(crate::error::ConstEvalNonIntError { span });
    |                              -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `IntoDiagnostic<'_>` is not implemented for `ConstEvalNonIntError`
    |                              required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
480 |     pub fn emit_err<'a>(&'a self, err: impl IntoDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`
Some errors have detailed explanations: E0277, E0308, E0432.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
