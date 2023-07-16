 rust
struct A;

impl A {
    fn test<'a>(v: Vec<&'a int>) {}
}

fn main() {
}
