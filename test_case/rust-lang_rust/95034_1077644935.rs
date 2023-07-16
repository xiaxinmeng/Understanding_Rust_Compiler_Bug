rust
pub trait Object<'a> { // <- can't remove `'a`
    type Error; // <- can't remove because then `E`s are no longer constrained below
    type Future;
}

impl Object<'static> for u8 {
    type Error = ();
    type Future = ();
}

impl<'a, E, A: Object<'a, Error = E>> Object<'a> for (A,) {
    type Error = ();
    type Future = CustomFut<'a, E, A>; // <- Replacing `E` with `A::Error` compiles
}

pub struct CustomFut<'a, E, A: Object<'a, Error = E>> {
    ph: std::marker::PhantomData<(A::Future,)>, // <- adding `E` to the tuple compiles
}

pub trait AsyncFn {
    fn call_async(&self);
}

impl<F: Fn() -> Fut, Fut> AsyncFn for F {
    fn call_async(&self) {}
}

async fn create<T: Object<'static>>() {
    let _fut: T::Future = unimplemented!();
    std::future::ready(()).await; // <- any await point should work
}

pub fn test() {
    let _ = create::<(u8,)>.call_async(); // <- .call_async() is important (can't remove or replace with `(...)()`; so is `(u8,)` (`u8` doesn't work)
}
