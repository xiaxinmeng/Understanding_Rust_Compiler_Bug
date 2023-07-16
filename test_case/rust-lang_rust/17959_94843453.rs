 rust
#![feature(unsafe_destructor)]

extern crate core;

use core::ops::Drop;

trait Bar {}

struct G<T: ?Sized> {
    _ptr: *const T
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
