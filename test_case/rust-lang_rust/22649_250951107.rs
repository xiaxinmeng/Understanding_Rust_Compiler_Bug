 rust
fn main() {
  let x = "a".to_string();
  // Doesn't work
  let y: String = ["b", &x, "c"].concat();
  println!("{}", y);
}
