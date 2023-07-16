 rust
struct A;

impl<'a> A {
    fn test(v: Vec<&'a int>) {}
}

fn main() {
}
