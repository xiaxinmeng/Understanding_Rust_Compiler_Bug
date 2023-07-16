 rust
fn main() {
  println!("{}", vec![1i8,2,3].iter().max_by(|n| n  + 0).map(|&e|e))
}
