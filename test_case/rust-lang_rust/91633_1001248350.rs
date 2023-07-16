rust
fn f<T> (it: &[T])
where
    [T] : std::ops::Index<usize>,
{
    let _ = &it[0];
}
