rust
fn safe_unwrap<T>(x: &Result<T, !>) -> &T {
    match x.as_ref() {
        Ok(x) => x,
    }
}
