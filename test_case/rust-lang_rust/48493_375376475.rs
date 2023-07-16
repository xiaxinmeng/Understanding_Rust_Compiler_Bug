rust
#![feature(box_syntax)]

use std::mem;

enum Void {}

struct RcBox<T> {
    _a: usize,
    _b: T,
}

pub unsafe fn bar() {
    mem::forget(box RcBox {
        _a: 1,
        _b: mem::uninitialized::<Void>(),
    });
}
