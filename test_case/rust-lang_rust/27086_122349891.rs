 Rust
use std::marker::PhantomData;
pub struct InvariantType<T>(PhantomData<*mut T>);
impl<T> Copy for InvariantType<T> {}
impl<T> Clone for InvariantType<T> { fn clone(&self) -> Self { *self } }
unsafe impl<T> Send for InvariantType<T> {}
unsafe impl<T> Sync for InvariantType<T> {}
