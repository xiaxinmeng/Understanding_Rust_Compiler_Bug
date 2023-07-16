 rust
fn enclose<A: Clone>(a: A) -> @fn() -> A {
  || a.clone()
}
fn main() {}
