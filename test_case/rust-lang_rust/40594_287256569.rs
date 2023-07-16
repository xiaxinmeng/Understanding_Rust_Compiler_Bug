rust
trait Foo {
    fn whatever() { }
}

trait Bar<P> {
    type X;
}

trait Baz { }
impl Baz for usize { }

impl<T, U> Foo for (T, U)
    where T: Bar<(T, U), X=U>
{ }

impl<T: Baz, P: Foo> Bar<P> for T {
    type X = u32;
}

fn main() {
    <(usize, u32) as Foo>::whatever();
}
