rust
use std::alloc::*;

#[global_allocator]
static ALLOC: A = A;

static mut HIT: bool = false;

struct A;

unsafe impl GlobalAlloc for A {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HIT = true;
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

fn main() {
    assert!(unsafe { HIT });
}
