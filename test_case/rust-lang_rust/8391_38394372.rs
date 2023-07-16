 rust
pub fn main() {
  let _x = match Some(1) {
    _y @ Some(_) => 1,
    None => 2,
  };
}
