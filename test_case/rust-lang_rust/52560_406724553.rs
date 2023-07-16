rust
struct Foo<A, B, C> {
    stuff: Pair<A, B>,
    thing: Builder<B>,
    marker: PhantomData<C>,
}
