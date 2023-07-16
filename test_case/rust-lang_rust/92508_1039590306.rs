rust
use std::ops::{Index, IndexMut};
use std::cell::Cell;

struct A<'a>(Cell<&'a mut ()>);

impl<'a> Index<&'a mut ()> for A<'a> {
    type Output = ();
    fn index(&self, idx: &'a mut ()) -> &Self::Output {
        &*self.0.replace(idx)
    }
}

impl<'a> IndexMut<&'a mut ()> for A<'a> {
    fn index_mut(&mut self, idx: &'a mut ()) -> &mut Self::Output {
        self.0.replace(idx)
    }
}

fn main() {
    let t = &mut ();
    let mut a = A(Cell::new(t));
    a[&mut ()] = ();
}
