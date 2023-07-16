rust
use std::marker::PhantomData;

pub struct Scope<'env> {
    _marker: PhantomData<&'env mut &'env ()>,
}
