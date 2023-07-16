rust
pub struct Foo<A: Default, B: Default>(A, B);

impl<A: Default, B: Default> Foo<A, B> {
    fn new(a: A) -> Self { Foo(a, B::default()) }
}

pub type Bar<A, B=usize> = Foo<A, B>;

fn main() {
    let bar = Bar::<_>::new(42usize);
}
