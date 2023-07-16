rust
#![feature(associated_consts)]

trait Trait {
  const N: usize;
}

fn n() -> usize {
  <Trait as Trait>::N
}
