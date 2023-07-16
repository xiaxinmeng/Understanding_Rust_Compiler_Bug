rust
#![feature(const_align_offset)]
use std::hint::unreachable_unchecked;

// Crate A
pub const fn inconsistent() -> usize {
    (&13 as *const i32).align_offset(2)
}

// Crate B
pub fn main() {
  const X: usize = inconsistent();
  let x = inconsistent();
  if x != X { unsafe { unreachable_unchecked(); }}
}
