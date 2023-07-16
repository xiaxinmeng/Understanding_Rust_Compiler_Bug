rust
#![feature(global_allocator, allocator_api)]

use std::heap::*;

#[global_allocator]
static A: B = B;

struct B;

static mut HIT: bool = false;

unsafe impl<'a> Alloc for &'a B {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        HIT = true;
        System.alloc(layout)
    }
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        HIT = true;
        System.dealloc(ptr, layout)
    }
}

fn main() {
    println!("hello!");
    assert!(unsafe { HIT });
}
