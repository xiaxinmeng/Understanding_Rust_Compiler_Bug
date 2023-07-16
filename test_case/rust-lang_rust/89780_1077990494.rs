rust
impl Poll<Option<Result<T, E>>> {
   fn bikeshed_transpose_option_result(self) -> Result<Poll<Option<T>>, E> { ... }
}
