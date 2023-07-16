rust
struct NotAFutureYet;

impl IntoFuture for NotAFutureYet {
    type Output = u8;
    type IntoFuture = Ready<Self::Output>;
    
    fn into_future(self) -> Self::IntoFuture {
        ready(42)
    }
}

async fn test() {
    assert_eq!(NotAFutureYet.await, 42);
}
