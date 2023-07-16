rust
#![feature(arbitrary_enum_discriminant)]

#[allow(dead_code)]
enum Enum {
  Unit = 3,
  Tuple() = 2,
  Struct {} = 1,
}

fn main() {
  assert_eq!(3, Enum::Unit as u8);
  assert_eq!(2, Enum::Tuple() as u8);
  assert_eq!(1, Enum::Struct{} as u8);
}
