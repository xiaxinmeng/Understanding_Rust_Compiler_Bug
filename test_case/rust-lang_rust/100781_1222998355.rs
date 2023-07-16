rust
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOC: Alloc = Alloc;

pub struct Alloc;

#[allow(unconditional_recursion)]
fn abort() -> ! {
    abort()
}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        abort()
    }

    unsafe fn alloc_zeroed(&self, _layout: Layout) -> *mut u8 {
        abort()
    }

    unsafe fn realloc(&self, _ptr: *mut u8, _layout: Layout, _new_size: usize) -> *mut u8 {
        abort()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        abort()
    }
}

fn main() {
    let _ = Box::new(1);
}
