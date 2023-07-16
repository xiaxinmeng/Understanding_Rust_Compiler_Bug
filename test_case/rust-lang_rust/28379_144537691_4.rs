 rust
fn main() {
  let x = 1 + {2} * {3};  // removed the unnecessary block here
  println!("{}", x);
}
