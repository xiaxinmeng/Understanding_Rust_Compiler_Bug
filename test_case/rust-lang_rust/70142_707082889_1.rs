rust
impl<T, E1, E2> for Result<Result<T, E1>, E2> {
  fn flatten_into<E = E1/E2>(self) -> Result<T, E> where E: From<E2>, E: From<E1> {
    Ok(self??)
  }
}
