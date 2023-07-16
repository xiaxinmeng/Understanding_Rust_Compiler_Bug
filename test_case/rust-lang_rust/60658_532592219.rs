rust
use std::future::Future;

pub trait Trait<'a> {
    type Assoc: Send + 'static;

    fn f(x: Self::Assoc) -> Box<dyn Future<Output = ()> + Send>
    where
        'a: 'static,
        Self: Sized + 'static,
    {
        Box::new(f::<Self>(x))
    }
}

async fn f<T: Trait<'static>>(_x: T::Assoc) {
    async {}.await
}
