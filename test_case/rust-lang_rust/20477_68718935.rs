 rust
impl<E: Error> FromError<E> for Box<Error> {
    fn from_error(e: E) -> Box<Error> {
        box e 
    } 
}
