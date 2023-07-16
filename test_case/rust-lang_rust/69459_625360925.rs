rust
struct Invariant<T>(fn(T) -> T);
pub struct Generic<T>(core::marker::PhantomData<Invariant<T>>);

impl<T> Generic<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
