`rust
fn main() {
  let mut x = String::from("constant size of 19");
  println!("size: {}, cap: {}", x.len(), x.capacity());
  x.push_str("const size of 16");
  println!("size: {}, cap: {}", x.len(), x.capacity());
}
