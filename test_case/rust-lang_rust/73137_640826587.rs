rust
#![feature(wake_trait)]
use std::future::Future;
use std::task::{Waker, Wake, Context};
use std::sync::Arc;


struct DummyWaker;
impl Wake for DummyWaker {
    fn wake(self: Arc<Self>) {}
}

struct Foo {
    a: usize,
    b: &'static u32,
}

fn main() {
    let mut fut = Box::pin(async {
        // Swap the order here and it works
        let action = Foo {
            b: &42,
            a: async { 0 }.await,
        };
        
        println!("{:p}", action.b);
        // ^ This prints `0x0`

        async {}.await;
    });
    let waker = Waker::from(Arc::new(DummyWaker));
    let mut cx = Context::from_waker(&waker);
    fut.as_mut().poll(&mut cx);
}
