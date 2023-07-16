rust
trait Foo: Iterator
where
    Self::Item: Default,
{
}

fn bar<T>(_: T)
where
    T: Foo,
    // Why the necessity of this bound? `Foo` already implies `Item: Default`
    T::Item: Default
{
}
