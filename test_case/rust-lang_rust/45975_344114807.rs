rust
#![feature(const_fn, const_unsafe_cell_new)]

use std::cell::UnsafeCell;

struct Test<T> {
    data: UnsafeCell<T>
}

impl<T> Test<T> {
    const fn new(data: T) -> Test<T> {
        Test { data: UnsafeCell::new(data) } 
    }
}

unsafe impl<T: Send> Sync for Test<T> {}

static FOO: Test<()> = Test::new(());

fn main() {}

