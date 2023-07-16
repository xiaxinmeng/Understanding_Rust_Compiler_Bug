rust
#![feature(const_transmute)]
#![warn(const_err)]

use std::mem;

const X: &i32 = unsafe {
    let x = 0;
    mem::transmute(&x)
};
