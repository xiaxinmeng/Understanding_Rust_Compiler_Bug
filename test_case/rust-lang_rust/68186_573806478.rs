rust
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {}
impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {}
