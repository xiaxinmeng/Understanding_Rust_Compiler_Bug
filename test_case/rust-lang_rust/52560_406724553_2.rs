rust
where // require each field to impl Debug
    Pair<A, B>: Debug,
    Builder<B>: Debug,
    PhantomData<C>: Debug,
