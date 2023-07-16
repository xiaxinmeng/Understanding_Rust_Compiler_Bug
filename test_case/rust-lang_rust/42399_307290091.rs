rust
#![feature(conservative_impl_trait)]

trait F {}

pub struct A<'a, T: 'a + F> {
    i: Vec<&'a [T]>,
    o: Vec<&'a [T]>,
}
impl<'a, T: 'a + F> A<'a, T> {
    pub fn z<'b: 'a>(&self) -> impl Iterator<Item = (&'b &[T], &'b &[T])> {
        self.i.iter().zip(self.o.iter())
    }
}

fn main() {}
