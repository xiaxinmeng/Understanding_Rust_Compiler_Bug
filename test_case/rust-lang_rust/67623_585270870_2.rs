rust
impl<'a, B> Baz for B 
where 
    B: Bar2<Foo<'a>>,
    B::X: Clone,
{}
