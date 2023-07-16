rust
#![feature(type_alias_impl_trait)]
use core::iter::empty;

#[cfg(not(doc))]
type Test = impl Iterator<Item = ()>;

#[cfg(doc)]
type Test = MockIterator<()>;
#[cfg(doc)]
struct MockIterator<T>(std::marker::PhantomType<T>);
#[cfg(doc)]
impl<T> Iterator for MockIterator<T> {
  type Item = T;
  fn next(&mut self) -> Self::Item { unreachable!() }
}

fn test() -> Test {
    empty()
}
