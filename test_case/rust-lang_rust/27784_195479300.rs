 rust
impl char {
    fn encode_utf8(&self) -> EncodeUtf8;
}

struct EncodeUtf8 {
    // ...
}

impl Iterator for EncodeUtf8 {
    type Item = u8;
    // ...
}

impl EncodeUtf8 {
    #[unstable(...)]
    pub fn as_slice(&self) -> &[u8] { /* ... */ }
}
