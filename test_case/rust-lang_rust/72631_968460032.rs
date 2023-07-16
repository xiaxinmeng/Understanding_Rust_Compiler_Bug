rust
pub trait Push {
  type Item;
  fn push<'a>(&'a mut self, item: Self::Item) -> &'a Self::Item;
}

pub trait TryPush {
  type Item;
  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<&'a Self::Item>;
}

pub trait TryExtend {
  type Item;
  fn try_extend<Iter: IntoIterator<Item = Self::Item>>(&mut self, iter: Iter) -> Result<()>;
}
