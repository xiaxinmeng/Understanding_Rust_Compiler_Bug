rust
use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::{Context, Poll},
};

pub struct SelectAll<Fut> {
    inner: Vec<Fut>,
}

pub struct Fuse<Fut> {
    inner: Option<Fut>,
}

impl<Fut: Future + Unpin> Future for SelectAll<Fut> {
    type Output = (Fut::Output, usize, Vec<Fut>);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}

async fn next_message() {
    todo!()
}

impl<Fut: Future> Future for Fuse<Fut> {
    type Output = Fut::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Fut::Output> {
        loop {}
    }
}

async fn run_one_step() {
    let mut all_next_messages = vec![Box::pin(next_message())];

    {
        {
            enum __PrivResult<_0> {
                _0(_0),
            }
            let __select_result = {
                let mut _0 = {
                    let this = {
                        let iter = all_next_messages.iter_mut();
                        let ret = SelectAll {
                            inner: iter.into_iter().collect(),
                        };
                        ret
                    };
                    Fuse { inner: Some(this) }
                };
                let mut __poll_fn = |__cx: &mut core::task::Context<'_>| {
                    let mut _0 = |__cx: &mut core::task::Context<'_>| {
                        let mut _0 = unsafe { core::pin::Pin::new_unchecked(&mut _0) };
                        if true {
                            None
                        } else {
                            Some(_0.poll(__cx).map(__PrivResult::_0))
                        }
                    };
                    let mut __select_arr = [&mut _0];
                    for poller in &mut __select_arr {
                        match poller(__cx) {
                            Some(x @ core::task::Poll::Ready(_)) => return x,
                            Some(core::task::Poll::Pending) => {}
                            None => {}
                        }
                    }

                    core::task::Poll::Pending
                };
                poll_fn(__poll_fn).await
            };
            match __select_result {
                __PrivResult::_0(_) => {
                    let _ = all_next_messages;
                }
            }
        }
    }
}
