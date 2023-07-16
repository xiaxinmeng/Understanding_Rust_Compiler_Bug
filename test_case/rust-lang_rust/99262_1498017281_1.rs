rust
impl Box<dyn Any> {
    pub fn downcast<T>(self) -> Result<Box<T>, Self>
    where
        T: Any
}

impl dyn Error {
    pub fn downcast<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>
    where
        T: Error + 'static
}
