rust
// this compiles
use std::borrow::Cow;

pub trait Trait {
    fn method(self) -> Cow<'static, [u8]>
    where
        Self: Sized;
}

impl Trait for Cow<'_, str> {
    fn method(self) -> Cow<'static, [u8]>
    where
        Self: Sized,
    {
        Cow::<'static, [u8]>::Borrowed(&[])
    }
}
