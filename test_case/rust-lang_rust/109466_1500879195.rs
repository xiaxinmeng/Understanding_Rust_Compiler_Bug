rust
use std::future::Future;

pub fn poll_recv() {
    let _: Box<dyn Future<Output = ()>> = Box::new(recv_unit());
}

async fn recv_unit() {
    std::future::ready(()).await;
}
