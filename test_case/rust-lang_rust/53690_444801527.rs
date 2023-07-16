rust
use futures::future::FutureObj;

async fn foo(x: bool) -> u32 {
    if x {
        let f = FutureObj::new(Box::new(foo(false)));
        await!(f) + 1
    }
    else {
        4
    }
}
