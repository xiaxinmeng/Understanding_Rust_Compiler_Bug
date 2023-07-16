rust
> impl<T, const N: usize> [T; N] {
>     pub fn try_split_array_ref<const M: usize>(&self) -> Option<(&[T; M], &[T])>;
> }
> 