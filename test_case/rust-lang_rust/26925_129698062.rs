 rust
#[derive(Clone)]
struct Item<A> { inner: A }
#[derive(Clone)]
pub struct IntoIter<A> { inner: Item<A> }
// The obvious bound is `Item<A> : Clone`, but we can't use that bound
// because `Item` is private.
