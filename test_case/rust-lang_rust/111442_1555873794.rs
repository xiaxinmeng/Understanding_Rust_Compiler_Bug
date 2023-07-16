rust
pub fn demo<T>(x: &[T], i: usize) -> Option<&T> {
    x.get(i)
}
