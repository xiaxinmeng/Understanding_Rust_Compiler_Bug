rust
// defined where `struct_lint_level` is:
struct LintDiagnosticBuilder<'a, 'b>(&'a mut DiagnosticBuilder<'b>);

impl<'a, 'b> LintDiagnosticBuilder<'a, 'b> {
    fn build(self, msg: &str) -> &'a mut DiagnosticBuilder<'b> {
        let LintDiagnosticBuilder(diag) = self);
        diag.set_message(msg);
        diag
    }
}
