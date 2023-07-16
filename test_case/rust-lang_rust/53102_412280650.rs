rust
// lib.rs

pub trait FromPointerReader<'a> : Sized {}

pub struct PointerReader<'a>  {
    _arena: &'a [u8],
}

pub trait OwnedTrait<'a> {
    type Reader: FromPointerReader<'a>;
}

pub struct Owned<T> where T: for<'a> OwnedTrait<'a> {
    marker: ::std::marker::PhantomData<<T as OwnedTrait<'static>>::Reader>,
}
