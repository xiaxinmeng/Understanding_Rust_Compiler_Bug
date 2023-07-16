 rust
#![crate_type = "lib"]

trait AsSlice {
    type Elem;

    fn as_slice(&self) -> &[Self::Elem];
}

trait PartialEq<T: ?Sized = Self> {
    fn eq(&self, &T) -> bool;
}

impl<T: AsSlice, U: AsSlice> PartialEq<T> for U where
    T::Elem: PartialEq<U::Elem>,
{
    fn eq(&self, _: &T) -> bool {
        true
    }
}
