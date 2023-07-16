rust
trait Foo<'a> {}

trait FooAll: for<'a> Foo<'a> {}
impl<T: ?Sized> FooAll for T where T: for<'a> Foo<'a> {}

trait Bar {
    type Item: FooAll;
}

fn foo<'a, T>(_: T)
where
    T: Bar,
    T::Item: Foo<'a>
{}
