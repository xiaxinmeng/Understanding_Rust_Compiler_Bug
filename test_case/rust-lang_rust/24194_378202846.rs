rust
#![feature(allocator_api)]
use std::heap::{Heap, Layout, Alloc};

pub unsafe fn alloc_zeroed_doesnt_optimise() {
    let _ = Heap.alloc_zeroed(Layout::from_size_align_unchecked(4, 8));
}

extern "C" {
    fn foo(x: *mut u8);
}

pub unsafe fn alloc_zeroed_should_optimise_rezeroing() {
    let a = Heap.alloc_zeroed(Layout::from_size_align_unchecked(16, 8)).unwrap();
    let slc = ::std::slice::from_raw_parts_mut(a, 16);
    for i in slc.iter_mut() {
        *i = 0;
    }
    foo(slc.as_mut_ptr());
}
