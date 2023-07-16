rust
pub struct Foo<A> {
    a: A,
}

pub struct Bar<A, F>
where
    F: AsRef<Foo<A>>,
{
    foo: F,
}
