 rust
pub struct QualifiedRule { ... }
pub struct AtRule { ... }
pub enum Rule {
    QualifiedRule(QualifiedRule),
    AtRule(AtRule),
}
