rust
use std::{future::Future, pin::Pin};

trait Foo: Send + Sync {
    type F: Future<Output = ()> + Send + 'static;
}

impl<R: Foo + ?Sized> Foo for Box<R> {
    type F = R::F;
}

fn a<R: Foo>() -> R::F { todo!() }
async fn b<R: Foo>() {
    a::<R>().await
}

fn c() -> impl Future<Output = ()> + Send {
    type A = Box<dyn Foo<F = Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>;
    Box::pin(async {
        b::<A>().await;
    })
}
