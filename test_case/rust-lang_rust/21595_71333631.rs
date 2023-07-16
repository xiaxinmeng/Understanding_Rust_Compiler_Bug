
fn main() {
  for i in -1u32..3 {
    if i % 2 == 0 {
      print!("*");
    }
    print!("{}",i);
  }
  println!("");
}
