 rust
trait NotSame {}
impl NotSame for .. {}
impl<T> !NotSame for (T, T) {}

trait Foo<A> {}

struct Bar<B>(B);

impl<A, B> Foo<A> for Bar<B> where (A, B): NotSame {
    // stuff
}

impl<A> Foo<A> for Bar<A> {
    // other stuff
}
