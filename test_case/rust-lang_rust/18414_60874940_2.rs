
impl<T> Result<T, Empty> {
    fn safe_unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => match e {},
        }
    }
}
