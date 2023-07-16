rust
where
    Foo: for<'a> Bar<'a>,
    for<'a> <Foo as Bar<'a>>::Assoc: Bound,
