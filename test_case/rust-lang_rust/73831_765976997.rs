rust
> impl<T> [T] {
>    pub fn partition_point<P>(&self, pred: P) -> usize
>     where
>         P: FnMut(&T) -> bool;
> }
> 