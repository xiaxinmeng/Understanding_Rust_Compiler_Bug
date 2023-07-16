rust
#[allow(dead_code)]
enum Enum {
  Unit,
  Tuple(),
  Struct{},
}

fn main() {
  assert_eq!(0, Enum::Unit as u8);
  assert_eq!(1, Enum::Tuple() as u8);
  assert_eq!(2, Enum::Struct{} as u8);
}
