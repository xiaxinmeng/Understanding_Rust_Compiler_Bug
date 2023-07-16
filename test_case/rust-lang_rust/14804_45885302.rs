 rust
enum LintPhase { AstOnly, Resolved, TypeChecked }

trait LintPass {
    fn initialise(phase: LintPhase, ...) -> Option<Self>
