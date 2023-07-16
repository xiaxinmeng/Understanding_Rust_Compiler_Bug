 rust
impl<'a, E: Error + 'a> FromError<E> for Box<Error + 'a> { }
