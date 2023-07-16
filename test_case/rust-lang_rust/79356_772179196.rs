rust
#![feature(generic_const_exprs)]

trait Foo where [(); Self::N]: {
  const N: usize;
}
