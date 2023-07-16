
impl<T, U> From<Box<T>> for Box<U>
where
  T: Unsize<U>
{
  fn from(value: Box<T>) -> Box<U> {
    value as Box<U>
  }
}
