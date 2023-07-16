`rust
fn main() {
  let mut x = vec![1, 2, 3, 4];
  println!("size: {}, cap: {}", x.len(), x.capacity());

  x.append(&mut vec![5, 6, 7, 8, 9, 10, 11]);
  println!("size: {}, cap: {}", x.len(), x.capacity());
}
