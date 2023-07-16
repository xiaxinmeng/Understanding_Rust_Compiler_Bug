 rust
fn rec(i: i32) {
  rec(i + 1)
}

fn main() {
  println!("{}", rec(0));
}
