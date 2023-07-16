rust
#[must_use]
#[derive(Clone)]
pub struct DiagnosticBuilder<'a>(Box<DiagnosticBuilderInner<'a>>);
