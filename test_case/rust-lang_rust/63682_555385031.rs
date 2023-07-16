rust
struct EscapeError {
    fallback: char,
    kind: EscapeErrorKind,
}

impl EscapeErrorKind {
    fn is_warning(&self) -> bool;
}
