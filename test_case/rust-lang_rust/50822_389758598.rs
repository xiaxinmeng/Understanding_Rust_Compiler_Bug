rust
pub struct Foo<A: Default, B = usize>(A, B);

impl<A: Default, B> Foo<A, B> {
    fn new(a: A) -> Self { Foo(a, panic!()) }
}

fn main() {
    let bar = Foo::new(42usize);
}
