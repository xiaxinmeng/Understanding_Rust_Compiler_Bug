rust
pub unsafe fn map<U, F>(this: Pin<'a, T>, f: F) -> Pin<'a, U>
