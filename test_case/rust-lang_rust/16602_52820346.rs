 rust
struct A {
  pub x: uint,
  pub y: uint
}

fn main() {
  let mut a = A { x: 1, y: 1 };
  a = A { x: a.y * 2, y: a.x * 2 };
  assert_eq!(a.x, 2);
  assert_eq!(a.y, 2);
}
