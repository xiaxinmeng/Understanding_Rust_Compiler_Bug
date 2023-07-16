rust
use futures::FutureExt; // 0.3.17
use std::future::Future;

fn foo() -> impl Future<Output = ()> + Send + Sync + 'static {
    async move {
        let work1 = async { "bang" };
        let work1 = work1.map(drop);
        let _error1 = futures::join!(work1);
    }
}
