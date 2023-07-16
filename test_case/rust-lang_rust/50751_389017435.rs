rust
declare_lint! {
    UNUSED_LOOP_LABEL,
    Warn,
    "warns on unused labels for loops",
}

#[derive(Copy, Clone)]
pub struct UnusedLoopLabel(Vec<Label>);

impl UnusedLoopLabel {
    // stateful lints are expected to have a `new()` constructor
    pub fn new() -> Self {
        UnusedLoopLabel(vec![])
    }
}

impl LintPass for UnusedLoopLabel {
    fn get_lints(&self) -> LintArray {
        lint_array!(UNUSED_LOOP_LABEL)
    }
}
