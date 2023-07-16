rust
#![no_std]
#![feature(allocator_api)]
#![feature(new_uninit)]

extern crate alloc;

use alloc::boxed::Box;
use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

pub struct Foo {}

unsafe impl Allocator for Foo {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        panic!();
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        panic!();
    }
}

impl Foo {
    pub fn foo(&self) {
        let _bar = Box::new_in(20, self);
    }
}
