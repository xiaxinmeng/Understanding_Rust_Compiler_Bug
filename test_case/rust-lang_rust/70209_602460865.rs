rust
#[derive(PartialEq, Eq)]
struct A;

impl A {
    const A: A = A;
}

fn main() {
    for <A>::A in vec![A] {}
}
