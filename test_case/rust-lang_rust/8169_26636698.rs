 rust
struct Outer {
  number: int
}

mod inner {
  use super::Outer;
  pub fn run_from_outside() {
    let _ = Outer { number: 42 };
  }
}

#[test]
fn test() {
  use inner::run_from_outside;
  run_from_outside();
}
