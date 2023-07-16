rust
enum Foo<'a> {
    Bar,
    Baz(&'a str),
}

impl<'a> Foo<'a> {
    fn to_owned(&self) -> Foo<'static> {
        use Foo::*;
        match self {
            Bar => Bar,
            Baz(x) => Baz(x.to_owned()),
        }
    }
}
