rust
type Result<T> = std::result::Result<T, Box<dyn Any + Send + 'static>>;
pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {…}
