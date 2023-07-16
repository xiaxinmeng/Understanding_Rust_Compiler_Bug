rust
pub fn struct_lint_level<'a>(
    sess: &'a Session,
    lint: &'static Lint,
    level: Level,
    src: LintSource,
    span: Option<MultiSpan>,
    decorate: impl Fn(LintDiagnosticBuilder<'_>),
) {
    // ...
}
