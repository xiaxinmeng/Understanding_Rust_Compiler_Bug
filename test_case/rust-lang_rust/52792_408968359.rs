rust
#![feature(async_await, await_macro, futures_api, arbitrary_self_types, pin)]

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

struct Stream;
impl Stream {
    async fn next(&mut self) -> Option<()> {
        Some(())
    }
}

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let fut = async move {
        let mut stream = Stream;
        loop {
            let next = stream.next();
            let opt = await!(next);
            let is_some = opt.is_some();
            if is_some {
                counter.fetch_add(1, Ordering::SeqCst);
            } else {
                break;
            }
        }
    };
}
