rust
impl<T, E1, E2> for Result<Result<T, E1>, E2>
where
  E1: From<E2>,
{
  fn flatten(self) -> Result<T, E1> {
    self?
  }
}
