rust
pub fn demo<T>(x: &[T]) -> Option<&T> {
    Some(x.split_first()?.0)
}
