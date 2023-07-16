rust
// `Future` would be implemented for this.
enum Awaitable<T> {
    NotFut(T),
    Fut(T)
}

trait ToAwaitable {
    type Out;
    fn awaitable(self) -> Awaitable<Self::Out>;
}

impl<T: !Future> ToAwaitable for T {
    type Out = T;

    fn awaitable(self) -> Awaitable<T> {
        // Would return Awaitable::NotFut
        unimplemented!()
    }
}

impl<T: Future> ToAwaitable for T {
    type Out = T::Output;

    fn awaitable(self) -> Awaitable<T::Output> {
        // Would return Awaitable::Fut
        unimplemented!()
    }
}
