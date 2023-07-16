rust
use std::marker::PhantomData;

pub mod traits {
    pub trait Owned<'a> {
        type Reader;
    }
}

pub struct Owned<T> where T: for<'a> ::traits::Owned<'a> {
    marker: PhantomData<<T as ::traits::Owned<'static>>::Reader>,
}
