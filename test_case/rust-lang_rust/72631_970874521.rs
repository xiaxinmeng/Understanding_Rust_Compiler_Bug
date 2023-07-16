rust
pub trait Push {
  type Item;
  type Output;
  fn push<'a>(&'a mut self, item: Self::Item) -> Self::Output;
}

pub trait TryPush {
  type Item;
  type Output;
  // the error should return the item cause it didn't consume it
  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::Output, ErrorFoo<Self::Item>>;
}
