rust
async fn __run1_task() {
    // stuff
}

fn run1() -> ::embassy_executor::SpawnToken<impl Sized> {
    type Fut = impl ::core::future::Future + 'static;
    static POOL: ::embassy_executor::raw::TaskPool<Fut, 1usize> = ::embassy_executor::raw::TaskPool::new();

    // defining use here. Signature for `_spawn_async_fn` is:
    // impl<F: Future + 'static, const N: usize> TaskPool<F, N> {
    //     fn _spawn_async_fn<FutFn>(&'static self, future: FutFn) -> SpawnToken<impl Sized>
    //          where FutFn: FnOnce() -> F;
    // }
    // so it constrains `Fut` to be the future for `__run1_task()`.
    unsafe { POOL._spawn_async_fn(move || __run1_task()) }
}
