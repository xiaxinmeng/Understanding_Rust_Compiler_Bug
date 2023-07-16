 rust
fn test (x: u32, y: u32) {
  match x {
    1 => println!("1"),
    _ if x==y => println!("y"),
    _ => println!("_"),
  };
}
