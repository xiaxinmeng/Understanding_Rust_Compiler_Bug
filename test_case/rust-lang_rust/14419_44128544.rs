 rust
trait WrapError<T> {
    fn wrap_error(e: T) -> Self;
}
impl<T> WrapError<T> for T {
    fn wrap_error(e: T) -> T {e}
}
fn wrap_error<T, E: WrapError<T> = T>(e: T) -> E {
    WrapError::wrap_error(e)
}
macro_rules! try(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => return Err(wrap_error(e)) })
)
