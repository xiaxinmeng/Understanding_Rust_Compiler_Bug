rust
pub struct DiagnosticBuilder {
    // Keep DiagnosticBuilder the way it currently is ...
}

// But introduce a new proxy type which stores just enough information that methods like
// struct_lint_level can set up things like level and span.
pub struct PendingDiagnosticBuilder<'a> {
    handler: &'a Handler,
    level: Level,
}

impl PendingDiagnosticBuilder<'_> {
    // Only accessible internally to rustc_errors.
    crate fn new(
        handler: &'a Handler,
        level: Level,
        span: Option<MultiSpan>,
    ) -> PendingDiagnosticBuilder<'a> {
        PendingDiagnosticBuilder { handler, level }
    }

    // Only method publicly accessible -- forces the user of the API to explicitly provide a
    // message the same way the API currently does.
    pub fn into_diagnostic_builder(self, msg: &str) -> DiagnosticBuilder {
        DiagnosticBuilder::new(handler, level, msg)
    }
}

// struct_lint_level & co get their signature changed to something like this:
pub fn struct_lint_level<'a>(
    sess: &'a Session,
    lint: &'static Lint,
    level: Level,
    src: LintSource,
    span: Option<MultiSpan>,
    set_message: impl Fn(PendingDiagnosticBuilder<'_>) -> DiagnosticBuilder<'_>,
) -> DiagnosticBuilder<'a> {
    // ...
}

// then, when calling struct_lint_level in another crate:
rustc::lint::struct_lint_level(session, lint, src, span, |pending_diagnostic| {
    pending_diagnostic.into_diagnostic_builder(format!(
        "Some diagnostic message which does string formatting: {}",
        something_expensive_to_display
    ))
});

// the API enforces that the caller provides a message. Because `PendingDiagnosticBuilder::new` and
// `DiagnosticBuilder::new` can't be called outside the crate, users of the API can't create their
// own, erroneous DiagnosticBuilder to return, either.

