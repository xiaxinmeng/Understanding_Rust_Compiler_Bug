 rust
fn main() {
  let mut u: ();
  let mut a: u8;

  u = (a = 2);  // right associativity
//(u = a) = 2;     left associativity, doesn't work
  u = a = 2;    // this works, so it must be right-associative

  print!("{} {}", a, u == ());
}
