rust
#![feature(core_intrinsics)]

enum Enum {
  A(u16) = 7,
  B(u32) = 42,
}

fn main() {
  use std::intrinsics::discriminant_value;

  unsafe {
    assert_eq!(discriminant_value(&Enum::A(1)), 7);
    assert_eq!(discriminant_value(&Enum::B(2)), 42);
  }
}
