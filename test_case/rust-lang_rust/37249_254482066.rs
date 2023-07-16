 rust
struct Foo<F, S> where F: Fn(S) {
    f: F,
    marker: ::std::marker::PhantomData<Fn(S)>,
}
