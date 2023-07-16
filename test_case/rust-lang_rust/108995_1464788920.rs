rust
pub trait B<I, O> {}
impl<T, I: ExternNotVoid> B<I, ()> for T {} // Generic over B<I, ()>
impl<T, O: ExternNotVoid> B<(), O> for T {} // I is generic,
// and () is !ExternNotVoid, so it can't be in I
// but that isn't factored into checking the validity of the implementation
