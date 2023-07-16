rust
impl std::io::Error {
    pub fn downcast<E>(self) -> Result<Box<E>, Self>
    where
        E: Error + Send + Sync + 'static
}
