plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0053]: method `into_diagnostic` has an incompatible type for trait
   --> compiler/rustc_session/src/errors.rs:50:36
    |
50  |     fn into_diagnostic(self, sess: &ParseSess) -> DiagnosticBuilder<'_, !> {
    |                                    |
    |                                    |
    |                                    expected struct `Handler`, found struct `ParseSess`
    |
note: type in trait
   --> compiler/rustc_session/src/session.rs:231:39
    |
    |
231 |     fn into_diagnostic(self, handler: &'a Handler) -> DiagnosticBuilder<'a, T>;
    |                                       ^^^^^^^^^^^
    = note: expected fn pointer `fn(TargetDataLayoutErrors<'_>, &Handler) -> DiagnosticBuilder<'_, _>`
               found fn pointer `fn(TargetDataLayoutErrors<'_>, &ParseSess) -> DiagnosticBuilder<'_, _>`
For more information about this error, try `rustc --explain E0053`.
error: could not compile `rustc_session` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_session` due to previous error
