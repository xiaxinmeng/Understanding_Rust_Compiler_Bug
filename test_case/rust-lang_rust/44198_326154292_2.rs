rust
#![feature(generators, generator_trait, conservative_impl_trait)]

use std::ops::{Generator, GeneratorState};

struct Tree<T> {
  left: Option<Box<Tree<T>>>,
  value: T,
  right: Option<Box<Tree<T>>>,
}

fn walk<T>(tree: &Tree<T>) -> Box<Generator<Yield=&T, Return=()>> {
    panic!()
}

fn main() {}
