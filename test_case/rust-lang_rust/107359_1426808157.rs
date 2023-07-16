rs
struct H<'a, T> {
    _marker: std::marker::PhantomData<&'a T>,
}

pub fn breaks<'a, F>(_: F)
    where F: core::ops::Fn(H<u8>) + 'a {}
