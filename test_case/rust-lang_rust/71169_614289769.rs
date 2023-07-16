
use std::mem::size_of;
use std::marker::PhantomData;

struct Raw<T: ?Sized> {
    d: [u8; size_of::<&T>()],
    _p: PhantomData<T>,
}
