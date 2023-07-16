rust
impl Error {
    pub fn into_inner(self) -> Option<Box<dyn Error + Send + Sync>> { ... }
}
