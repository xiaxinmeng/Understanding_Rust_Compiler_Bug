rust
> impl<T, const N: usize> [T; N] {
>     fn split_array<const M: usize>(&self) -> (&[T; M], &[T; {N - M}], ...)
> }
> 