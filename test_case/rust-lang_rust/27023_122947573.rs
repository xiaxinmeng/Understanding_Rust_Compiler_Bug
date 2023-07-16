 rust
#![feature(no_std, lang_items, start, box_syntax, unsize, unique, core_prelude)]
#![feature(core, coerce_unsized, core_intrinsics, libc, linkage)]
#![no_std]
#![allow(improper_ctypes)]

#[macro_use]
extern crate core;
extern crate libc;

use core::prelude::*;
use core::ptr::Unique;
use core::marker::Unsize;
use core::ops::CoerceUnsized;

struct Wrapper<'a, T: ?Sized>(&'a mut i32, T);

impl<'a, T: ?Sized> Drop for Wrapper<'a, T> {
    fn drop(&mut self) {}
}

#[lang = "owned_box"]
struct Box<T>(Unique<T>);
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<Box<U>> for Box<T> {}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let mut x = 3;
    let wrapper: Box<Wrapper<i32>> = box Wrapper(&mut x, 123);
    let _: Box<Wrapper<Send>> = wrapper;
    0
}

#[lang = "exchange_malloc"]
unsafe fn exchange_malloc(size: usize, align: usize) -> *mut u8 {
    assert_eq!(size, 16);
    assert_eq!(align, 8);
    extern {
        fn malloc(size: usize) -> *mut u8;
    }
    malloc(size)
}

#[lang = "exchange_free"]
unsafe fn exchange_free(ptr: *mut u8, old_size: usize, align: usize) {
    assert_eq!(old_size, 16);
    assert_eq!(align, 8);
    extern {
        fn free(ptr: *mut u8);
    }
    free(ptr)
}



#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
#[linkage = "external"]
pub fn panic_fmt() -> ! {
    unsafe { core::intrinsics::abort() }
}
