rust
pub enum Cow<'a, B: ?Sized + 'a>
    where B: ToOwned
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

pub struct Foo<'a>(Cow<'a, [Foo<'a>]>);
