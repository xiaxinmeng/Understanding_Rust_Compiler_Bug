rust
use std::ops::{Index, IndexMut};
use std::cell::Cell;

struct A<'a>(Cell<&'a mut ()>);

const UNIT: () = ();

impl<'a> Index<usize> for A<'a> {
    type Output = ();
    fn index(&self, _: usize) -> &Self::Output {
        &UNIT
    }
}

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
    a[0]; // <- same error here
}

// Similarly, even without the `a[0];` line, implementing Drop also leads to the same compilation error.
impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("bye")
    }
}
