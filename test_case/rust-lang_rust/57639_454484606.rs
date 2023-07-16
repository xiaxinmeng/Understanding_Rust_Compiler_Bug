rust
trait Foo<'a> {}

trait Bar {
    type Item: for<'a> Foo<'a>;
}

fn foo<'a, T>(_: T)
where
    T: Bar,
    T::Item: Foo<'a>
{}
