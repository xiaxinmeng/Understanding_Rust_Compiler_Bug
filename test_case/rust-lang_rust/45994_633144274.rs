
impl StructWithCallback {
    // This works, but requires a macro/something to convert from `async fn` to something that has the right signature. 
    fn new(cb: for<'r> fn(&'r mut MyStruct) -> Pin<Box<dyn Future<Output = ()> + 'r>>) -> Self {
        Self {
            callback: Box::new(move |s| cb(s)),
        }
    }
    // I believe that this would also work if we allowed `impl Future` in this position.
    fn from_async_fn(cb: impl for<'r> FnMut(&'r mut MyStruct) -> (impl Future<Output = ()> + 'r)) -> Self {
        Self {
            callback: Box::new(move |s| Box::pin(cb(s))),
        }
    }
}
