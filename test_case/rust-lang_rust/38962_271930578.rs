`rust
    BarOwned(Vec<Foo<'static>>),
    BarBorrowed(&'a [Foo<'a>]),
