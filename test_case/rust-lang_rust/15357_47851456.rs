 rust
fn test<'a>(s: &'a str) -> &'a str {
    s.as_slice()
}
