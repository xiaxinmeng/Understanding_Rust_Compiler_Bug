rust
use core::ops::Deref;

pub struct Nested<Outer> {
    outer: Outer,
}

impl<Outer> Deref for Nested<Outer>
    where
        Outer: Deref,
        Outer::Target: Deref,
{
    type Target = <Outer::Target as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.outer
    }
}
