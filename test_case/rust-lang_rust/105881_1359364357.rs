rust
// check-fail
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct SelectAll<Fut> {
    _inner: Fut,
}

impl<Fut: Future> Future for SelectAll<Fut> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}

async fn new_future() {
    loop {}
}

fn select_all<I>(_: I) -> SelectAll<I::Item>
where
    I: IntoIterator,
    I::Item: Future + Unpin,
{
    loop {}
}

async fn run_one_step() {
    let mut select = select_all(vec![Box::pin(new_future())]);
    let mut poll_fn = |cx: &mut Context<'_>| Pin::new(&mut select).poll(cx);
    for _poll_fn in [&mut poll_fn] {}
}

fn main() {}
