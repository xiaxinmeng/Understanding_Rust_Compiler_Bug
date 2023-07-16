rust
pub fn push_in_capacity<T>(v: &mut Vec<T>, x: T) -> Result<(), T> {
    if v.len() == v.capacity() {
        Err(x)
    } else {
        v.push(x);
        Ok(())
    }
}
