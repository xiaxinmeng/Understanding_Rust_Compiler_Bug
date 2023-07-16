rust
use core::iter::Sum;

struct A;
struct B;
#[derive(Clone, Copy)]
struct C;

impl<'a> Sum<&'a C> for A {
    fn sum<I>(_: I) -> Self where I: Iterator<Item = &'a C> {
        A
    }
}

impl<'a> Sum<&'a C> for B {
    fn sum<I>(_: I) -> Self where I: Iterator<Item = &'a C> {
        B
    }
}

fn main() {
    let c = [C; 5];
    let _a: A = c.iter().sum();
    let _b: B = c.iter().sum();
}
