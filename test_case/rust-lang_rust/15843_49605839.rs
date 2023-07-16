
trait Foo<L> {
    fn foo() -> L;
}

pub fn bar<L, T: Foo<L>>(data: T) -> L {
    data.foo()
}

fn main() {}
