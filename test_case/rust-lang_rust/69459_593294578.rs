rust
pub struct Generic<T>(core::marker::PhantomData<fn(T) -> T>);

impl<T> Generic<T> {
    const NEW: Self = Self(core::marker::PhantomData);
}

impl<T> Generic<T> {
    pub const fn new() -> Self {
        Self::NEW
    }
}
