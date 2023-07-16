rust
use core::pin::Pin;
use std::future::Future;

pub trait Foo<'a> {
    type Future: Future<Output = ()>;
    
    fn foo() -> Self::Future;
}

struct MyType<T>(T);

impl<'a, T> Foo<'a> for MyType<T>
where
    T: Foo<'a>,
    T::Future: Send,
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    
    fn foo() -> Self::Future {
        Box::pin(async move {
            T::foo().await
        })
    }
}
