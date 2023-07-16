rust
pub unsafe fn map<U, F>(this: &'b mut Pin<'a, T>, f: F) -> Pin<'b, U>
