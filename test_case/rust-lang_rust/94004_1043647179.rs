rust
use core::future::Future;

trait Callback<'a> {}

impl<'a, T, Fut> Callback<'a> for T
where
    T: FnOnce(&'a mut ()) -> Fut,
    Fut: Future<Output = ()> + 'a,
{
}

async fn transaction<Cb>(f: Cb)
where
    Cb: for<'a> Callback<'a>,
{
}

async fn my_callback(bus: &mut ()) {}

async fn boom() {
    transaction(my_callback).await;
}
