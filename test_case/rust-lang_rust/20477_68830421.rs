 rust
impl <T> error::FromError<sync::PoisonError<T>> for InitializationError {
    fn from_error(error: sync::PoisonError<T>) -> InitializationError {
        InitializationError::Other(format!("{}",error))
    }
}
