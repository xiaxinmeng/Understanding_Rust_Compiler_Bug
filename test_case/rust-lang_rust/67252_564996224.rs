rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

fn spawn<T: Send>(_: T) {}
    
pub struct AFuture;
impl Future for AFuture{
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        unimplemented!()
    }
}

async fn foo() {
    spawn(async {
        let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
        AFuture.await;
    });
}

fn main() {}
