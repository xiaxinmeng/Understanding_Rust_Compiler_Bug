rust
struct YeetSuccess<R>(R);

impl<T, E> Result<T, E> {
    /// Wraps this result in a type such that, when applied with `?`, 
    /// will either `return Ok(t)` early or extract the error value out.
    pub fn yeet_ok(self) -> YeetSuccess<Self> {
        YeetSuccess(self)
    }
}

impl<T> Option<T> {
    /// Wraps this result in a type such that, when applied with `?`, 
    /// will either `return Some(t)` early or results in an empty value.
    pub fn yeet_some(self) -> YeetSuccess<Self> {
        YeetSuccess(self)
    }
}
