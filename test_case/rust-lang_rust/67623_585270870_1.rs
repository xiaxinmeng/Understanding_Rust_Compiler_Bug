rust
impl<'a, A, B> Baz for B 
where 
    B: Bar2<Foo<'a>, X=A>,
    A: Clone,
{}
