rust
pub struct RangeWrapperBecauseOfDefaultDomain<T>(pub core::ops::Range<T>);

impl<T> Default for RangeWrapperBecauseOfDefaultDomain<T>
where
  T: Default
{
  fn default() -> Self {
    Self(T::default()..T::default())
}
