rust
use std::task::{pend, Context, Poll};
use std::future::{self, Future};
use std::pin::Pin;

pub fn do_poll(cx: &mut Context<'_>) -> Poll<()> {
    let mut fut = future::ready(42);
    let fut = Pin::new(&mut fut);

    let num = if_ready!(fut.poll(cx));
    // ... use num

    Poll::Ready(())
}
