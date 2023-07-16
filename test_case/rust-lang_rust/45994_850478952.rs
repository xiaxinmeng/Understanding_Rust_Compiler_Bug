rust
use std::iter::Rev;
use std::slice::IterMut;

trait MakeIter<'a> {
    type Iter: Iterator<Item = &'a mut i32>;

    fn make_iter(&mut self, slice: &'a mut [i32]) -> Self::Iter;
}

fn foo(mut make_iter: impl for<'a> MakeIter<'a>) {
    let mut data = [1, 2, 3, 4];

    make_iter.make_iter(&mut data);
}

struct Forward;

impl<'a> MakeIter<'a> for Forward {
    type Iter = IterMut<'a, i32>;

    fn make_iter(&mut self, slice: &'a mut [i32]) -> Self::Iter {
        slice.iter_mut()
    }
}

struct Backward;

impl<'a> MakeIter<'a> for Backward {
    type Iter = Rev<IterMut<'a, i32>>;

    fn make_iter(&mut self, slice: &'a mut [i32]) -> Self::Iter {
        slice.iter_mut().rev()
    }
}

fn main() {
    foo(Forward);
    foo(Backward);
}
