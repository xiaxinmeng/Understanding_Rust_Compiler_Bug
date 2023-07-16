plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: `#[error(...)]` is not a valid attribute
    |
338 | #[error(parser::invalid_variable_declaration)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
338 | #[error(parser::invalid_variable_declaration)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
   --> compiler/rustc_parse/src/parser/diagnostics.rs:338:3
    |
338 | #[error(parser::invalid_variable_declaration)]
---
11  | #[macro_use]
    | ^^^^^^^^^^^^

    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0277]: the trait bound `InvalidVariableDeclaration: SessionDiagnostic<'_>` is not satisfied
    |
    |
63  |               self.sess.emit_err(InvalidVariableDeclaration {
    |  _______________________--------_^
    | |                       required by a bound introduced by this call
64  | |                 span: mut_let_span,
64  | |                 span: mut_let_span,
65  | |                 sub: InvalidVariableDeclarationSub::SwitchMutLetOrder(mut_let_span),
66  | |             });
    | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidVariableDeclaration`
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:299:45
    |
    |
299 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `InvalidVariableDeclaration: SessionDiagnostic<'_>` is not satisfied
    |
    |
219 |         self.sess.emit_err(InvalidVariableDeclaration { span: lo, sub: subdiagnostic(lo) });
    |                   -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidVariableDeclaration`
    |                   required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:299:45
   --> /checkout/compiler/rustc_session/src/parse.rs:299:45
    |
299 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_parse` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_parse` due to 5 previous errors
