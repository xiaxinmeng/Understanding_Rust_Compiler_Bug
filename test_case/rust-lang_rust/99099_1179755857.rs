rust
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct First;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Last;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneContext<Behavior, Context> {
  behavior: PhantomData<Behavior>,
  context: Context,
}

impl<Context> BitOr for OneContext<First, Context> {
  type Output = Self;

  fn bitor(self, _rhs: Self) -> Self::Output {
    self
  }
}

impl<Context> BitOr for OneContext<Last, Context> {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    rhs
  }
}
