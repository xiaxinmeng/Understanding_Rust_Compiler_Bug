rust
use core::iter;

pub trait ToIter<T> {
    type Iter<'b>: Iterator<Item = &'b T>
    where
        T: 'b;

    fn iter(&self) -> Self::Iter<'_>;
}

pub struct DrainRange<'a, T>(&'a T);

impl<'a, T> ToIter<T> for DrainRange<'a, T> {
    type Iter<'b>
    where
        T: 'b,
    = iter::FromFn<impl FnMut() -> Option<&'b T>>;

    fn iter(&self) -> Self::Iter<'_> {
        unimplemented!()
    }
}

fn main() {}
