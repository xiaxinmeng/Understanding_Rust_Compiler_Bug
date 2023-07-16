plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0053]: method `into_diagnostic` has an incompatible type for trait
   --> compiler/rustc_session/src/session.rs:238:16
    |
238 |         _sess: &'a ParseSess,
    |                |
    |                |
    |                expected struct `Handler`, found struct `ParseSess`
    |                help: change the parameter type to match the trait: `&'a Handler`
note: type in trait
   --> compiler/rustc_session/src/session.rs:232:39
    |
    |
232 |     fn into_diagnostic(self, handler: &'a Handler) -> DiagnosticBuilder<'a, T>;
    |                                       ^^^^^^^^^^^
    = note: expected fn pointer `fn(Spanned<_>, &'a Handler) -> DiagnosticBuilder<'_, _>`
               found fn pointer `fn(Spanned<_>, &'a ParseSess) -> DiagnosticBuilder<'_, _>`
For more information about this error, try `rustc --explain E0053`.
error: could not compile `rustc_session` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_session` due to previous error
