rust
fn main() {
  let a = std::sync::Mutex::new(5);
  let b = a.lock().unwrap();
  println!("{:#?}", a);
}
