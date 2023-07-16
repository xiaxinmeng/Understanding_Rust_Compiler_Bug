rust
use std::marker::PhantomData;

pub trait OwnedTrait<'a> {
    type Reader;
}

pub struct Owned<T> where T: for<'a> OwnedTrait<'a> {
    marker: PhantomData<<T as OwnedTrait<'static>>::Reader>,
}
