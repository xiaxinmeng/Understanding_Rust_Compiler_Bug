rust
> impl<T, E> Result<T, Result<T, E>> {
>       pub fn flatten(self) -> Result<T, E>;
> }
> 