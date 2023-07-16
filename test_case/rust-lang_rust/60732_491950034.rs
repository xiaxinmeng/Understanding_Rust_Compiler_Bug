rust
#![feature(arbitrary_enum_discriminant)]

#[repr(u8)]
#[allow(dead_code)]
enum Enum {
  Unit = 3,
  Tuple(()) = 2,
  Struct {
    a: (),
    b: (),
  } = 1,
}

fn main() {
  println!("{:?}", Enum::Unit as u8);
}
