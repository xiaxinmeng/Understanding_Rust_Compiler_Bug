rust
> impl<T, E> Result<Result<T, E>, E> {
>     pub fn flatten(self) -> Result<T, E>;
> }
> 
> impl<T, E1, E2> Result<Result<T, E1>, E2> {
>     pub fn flatten_into<E>(self) -> Result<T, E> where E1: Into<E>, E2: Into<E>;
> }
> 