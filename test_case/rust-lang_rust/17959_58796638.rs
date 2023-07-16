
#![feature(unsafe_destructor)]

extern crate core;

use core::ops::Drop;


trait Bar {}

struct GBox<Sized? T> {
    value: T
}

struct G<Sized? T> {
    _ptr: *mut GBox<T>
}

#[unsafe_destructor]
impl<T> Drop for G<T> {
    fn drop(&mut self) {
        if !self._ptr.is_null() {
        }
    }
}

fn main() {
    let x:G<Bar>;
}
