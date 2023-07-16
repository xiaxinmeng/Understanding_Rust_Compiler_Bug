rust
#[async_trait::async_trait]
impl<T> Trait<T> for Struct<T> {
    fn foo(&self) -> Pin<Box<dyn Future<Output = Res>,>> {
        async fn run(this: Struct<T>) -> Res {
            unimplemented!()
        }
        unimplemented!()
    }
}
