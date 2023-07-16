
fn enclose<A:Copy>(a: A) -> @fn() -> A {
  || copy a
}

fn main() { }
