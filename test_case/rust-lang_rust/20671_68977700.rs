 rust
trait Foo<T> {
    fn foo(&self) -> &T;
}

//trait Bar<A> where Self: Foo<A> {}  <--- this produces an error
trait Bar<A>: Foo<A> {}

fn foobar<A, B: Bar<A>>(b: &B) -> &A {
    b.foo()
}
