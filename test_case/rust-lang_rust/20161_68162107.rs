 rust
#[inline(never)]
fn wrap<T>(v: T) -> Option<T> {
    Some(v)
}
