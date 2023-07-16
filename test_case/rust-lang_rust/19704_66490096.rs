 rust
fn main() {
 let y = {
  // "hello world" allocated
  "hello world".to_string().as_slice().clone()
  // "hello world" deallocated
 };
 // new string allocated in same spot
 let z = "something".to_string();
 println!("{}", y)
 // prints "somethingld"
}
