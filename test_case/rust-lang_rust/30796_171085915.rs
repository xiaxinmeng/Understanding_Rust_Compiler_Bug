 Rust
#[stable(feature = "box_error", since = "1.7.0")]
impl<T: Error + ?Sized> Error for Box<T> {
    fn description(&self) -> &str {
        Error::description(&**self)
    }

    fn cause(&self) -> Option<&Error> {
        Error::cause(&**self)
    }
}
