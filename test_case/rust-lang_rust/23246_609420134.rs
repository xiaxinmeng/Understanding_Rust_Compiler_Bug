rust
pub enum Empty {}
pub enum Foo<A, B> {
    X(A),
    // ...
    Void(Empty, PhantomData<B>),
}
