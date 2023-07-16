rust
struct InvariantUniqueToken<'a>  { inner: PhantomData<fn(&'a()) -> &'a ()> }
