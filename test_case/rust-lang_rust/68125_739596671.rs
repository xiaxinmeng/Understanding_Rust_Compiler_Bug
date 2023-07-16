rust
fn fold_first_vec<T>(mut v: Vec<T>, combine: fn(T, T) -> T) -> Option<T> {
    let last = v.pop()?;
    Some(v.into_iter().fold(last, combine))
}
