 rust
> impl<'a, T> Index<Range<usize>> for Iter<'a, T>
>   type Output = [T]
>   fn index(&self, index: &Range<usize>) -> &[T]
> 