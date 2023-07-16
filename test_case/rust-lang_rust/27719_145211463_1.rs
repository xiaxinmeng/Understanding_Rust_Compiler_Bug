 rust
type Result<T> = Result<T, Box<Any + Send + 'static>>;
