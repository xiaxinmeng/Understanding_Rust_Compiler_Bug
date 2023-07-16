rust
#![feature(const_generics)]

pub trait Trait<const N: usize> {
  type Item;
  const VAL: <Self as Trait<N>>::Item;
}
