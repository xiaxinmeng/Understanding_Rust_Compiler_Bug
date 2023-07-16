rust
#![feature(unboxed_closures)]

use std::future::Future;

use futures::future::LocalBoxFuture;


pub trait GenericTask<C>: 'static {
    fn execute<'a>(&'a self, ctx: &'a C) -> LocalBoxFuture<'a, ()>;
}

impl<C: 'static, T: 'static> GenericTask<C> for T
where
    T: for<'a> Fn<(&'a C, )>,
    for<'a> <T as FnOnce<(&'a C, )>>::Output: Future<Output = ()> + 'a,
{
    fn execute<'a>(&'a self, ctx: &'a C) -> LocalBoxFuture<'a, ()> {
        let future = (self)(ctx);

        Box::pin(async move {
            future.await;
        })
    }
}
