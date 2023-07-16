rust
pub fn test(s: Option<&String>) -> Option<&str> {
    match s.as_ref() {
        Some(s) => Some(s),
        None => None,
    }
}
