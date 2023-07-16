rust
impl String {
    pub fn leak(s: impl Into<String>) -> &'a mut str {}
}
