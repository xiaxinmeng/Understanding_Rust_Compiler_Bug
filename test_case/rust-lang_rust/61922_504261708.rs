rust
fn main() {
  static || {
    let x = String::new("42");
    let y = x;
    yield;
    assert!(&x == "42");
  }
}
