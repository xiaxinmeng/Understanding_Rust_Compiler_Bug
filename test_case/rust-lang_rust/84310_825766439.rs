rust
> impl<T: LambdaL> ScopedCell<T> {
>     #[rustc_allow_const_fn_unstable(const_fn)]
>     pub const fn new(value: <T as ApplyL<'static>>::Out) -> Self {
>         ScopedCell(Cell::new(value))
>     }
> }
> 