rust
struct Foo<T>(T);

trait FooLike { type Output; }
impl<T> FooLike for Foo<T> {
    type Output = T;
}


trait Trait {
    type Assoc;
}

fn foo<T: Trait<Assoc = ()>>() -> impl FooLike<Output = T::Assoc> {
    Foo(())
}
