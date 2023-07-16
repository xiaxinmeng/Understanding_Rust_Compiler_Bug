 rust
impl char {
    fn encode_utf8(&self) -> Utf8Char;
}

struct Utf8Char {
    // ...
}

impl Deref for Utf8Char {
    type Target = [u8];
    fn deref(&self) -> &[u8] { /* ... */ }
}
