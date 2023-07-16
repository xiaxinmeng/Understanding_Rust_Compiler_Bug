rust
fn g<T>(t: T) -> Result<impl Iterator<Item=u32>, T> {
    Err(t)
}
