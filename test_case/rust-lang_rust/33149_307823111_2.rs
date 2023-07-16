Rust
// ok because String is local, so we know `!String: ErrorOrStr`
impl From<String> for Box<Error + Send + Sync> {
    fn from(s: String) -> Self {
        Self::from(s.into_boxed_str())
    }
}

// ok because String is local, so we know `!String: ErrorOrStr`
impl From<String> for Box<Error> {
    fn from(s: String) -> Self {
        Self::from(s.into_boxed_str())
    }
}
