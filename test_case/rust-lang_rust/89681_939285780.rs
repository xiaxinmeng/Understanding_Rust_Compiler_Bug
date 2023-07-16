rust
use std::future::Future;

pub trait Handler<T> {}

impl<F, Fut> Handler<()> for F
where
    F: FnOnce() -> Fut,
    Fut: Future,
{
}

impl<F, Fut, T1> Handler<(T1,)> for F
where
    F: FnOnce(T1) -> Fut,
    Fut: Future,
{
}

#[test]
fn test() {
    fn check_handler<H, T>(_: H)
    where
        H: Handler<T>,
    {
    }

    check_handler(|| {});
}
