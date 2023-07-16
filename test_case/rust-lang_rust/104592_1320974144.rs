rust
trait Foo {
    fn foo(&self) -> impl Future<Output = ()> + '_ + Send;
}

impl Foo for () {
    async fn foo(&self) { }
}
