rust
impl [T] {
    fn split_at_array<const N: usize>(&self) -> (&[T; N], &[T])
}
