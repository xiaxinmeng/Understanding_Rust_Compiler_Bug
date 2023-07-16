rust
pub trait FooExt: Foo {
    fn run_via<'a, 'b: 'a, T: Sync>(&'a self, t: &'b T) -> impl Future<Output = ()> + 'a + Send {
        async move {
            self.run::<'a, 'b, T>(t).await;
        }
    }
}
