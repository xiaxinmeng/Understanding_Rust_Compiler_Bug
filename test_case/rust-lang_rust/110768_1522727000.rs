rust
use core::ops::Deref;

pub struct Nested<Outer> {
    outer: Outer,
}

impl<T, Outer> Deref for Nested<Outer>
where
    Outer: Deref,
    Outer::Target: Deref<Target = T>,
    T: ?Sized,
{
    type Target = T;

    #[inline]
    fn deref<'a>(&'a self) -> &'a T {
        let outer: &'a Outer = &self.outer;
        let inner: &'a Outer::Target = Deref::deref(outer);
        let x: &'a T = Deref::deref(inner);
        x
    }
}
