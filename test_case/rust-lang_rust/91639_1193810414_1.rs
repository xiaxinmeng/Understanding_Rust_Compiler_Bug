rust
impl<F> Foo for F
where
    for<'a> F: Fn(A<'a>),
{}
