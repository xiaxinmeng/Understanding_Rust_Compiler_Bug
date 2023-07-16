rust
trait Foo {
    fn foo<'me, 'x>(&'me self, x: &'x u32) -> impl Future<Output = ()> + 'me + 'x;
}

impl Foo for () {
    async fn foo(&self, x: &u32) {}
    // ^ error[E0495]: cannot infer an appropriate lifetime for lifetime parameter '_ in generic type due to conflicting requirements
}
