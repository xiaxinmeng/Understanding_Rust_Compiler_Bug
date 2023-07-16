rust
#![feature(const_generics)]

pub trait Trait<const N: usize>: From<<Self as Trait<N>>::Item> {
  type Item;
}
