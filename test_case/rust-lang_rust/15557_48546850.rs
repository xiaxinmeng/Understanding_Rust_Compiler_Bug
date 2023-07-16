 rust
use std::ty::Unsafe;

struct AReg1<'a>(&'a u32);

impl<'a> Drop for AReg1<'a> {
  fn drop(&mut self) {}
}
