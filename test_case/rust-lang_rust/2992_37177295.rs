 rust
fn enclose<A: Clone>(a: A) -> proc() -> A {
  proc() a.clone()
}
fn main() {}
