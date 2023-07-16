rust
fn my_remove<T>(v: &mut Vec<T>, i: usize) -> Option<T> {
    v.get(i)?;
    Some(v.remove(i))
}
