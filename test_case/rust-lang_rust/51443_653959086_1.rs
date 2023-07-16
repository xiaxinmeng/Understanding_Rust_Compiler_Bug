rust
impl Advertisement for AutoplayingVideo {
    fn run<'async>(
        &'async self,
    ) -> Pin<Box<dyn core::future::Future<Output = ()> + Send + 'async>>
    where
        Self: Sync + 'async,  // <--------- AHA?
    {
        // ...
    }
}
