rust
#[unstable(feature = "proc_macro_diagnostic", issue = "38356")]
pub fn children(&self) -> impl Iterator<Item = &Diagnostic> { .. }
