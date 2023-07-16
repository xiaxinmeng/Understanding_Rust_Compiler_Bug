 rust
pub fn main() {
  let a: int = 0;
  let b: &int = &a;
  let c: & &int = &b;
  let b: int = 1;
  println!("**c = {}", **c);
  println!("b = {}", b);
}
