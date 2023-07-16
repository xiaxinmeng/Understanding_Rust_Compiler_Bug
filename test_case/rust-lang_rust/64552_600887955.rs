rust
use futures::{
    TryStream,
    Stream,
    stream::{iter},
    StreamExt,
};
use std::future::Future;
use std::task::{Poll, Context};
use std::pin::Pin;

pub(crate) type BoxedFutureTask = Pin<Box<dyn Future<Output = ()> + 'static + Send>>;

pub fn require_send<T: Send>(_: T) {}

struct MyFut<V>(V::Ok) where V: TryStream;
impl<V, Good, Bad> Future for MyFut<V> where
    V: Stream<Item = Result<Good, Bad>> {
    
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        panic!()
    }
}


fn my_send_all<St>(stream: &mut St) -> MyFut<St> where St: TryStream<Error = ()>
{
    panic!()
}
   
async fn dummy() {}

async fn inner() {
    let mut tasks = iter(
        std::iter::once(Ok(Box::pin(async {}) as BoxedFutureTask))
    ).map(|val| val);
    my_send_all(&mut tasks).await;
}

fn main() {
    require_send(inner())
}
