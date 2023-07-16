rust
use core::cmp::PartialEq;

#[derive(Clone, Eq)]
pub struct Struct<T: Clone>(T);

impl<T, U> PartialEq<U> for Struct<T>
where
    U: Into<Struct<T>> + Clone, T: Clone
{
    fn eq(&self, other: &U) -> bool {
        todo!()
    }
}
