rust
use std::alloc::*;
use std::process::Command;
use std::env;

#[global_allocator]
static ALLOC: A = A;

static mut HITS: usize = 0;

struct A;

unsafe impl GlobalAlloc for A {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HITS += 1;
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

fn main() {
    unsafe {
        let start = HITS;
        Box::new(2); // our own crate makes an allocation
        assert_eq!(HITS, start + 1);
        drop(env::var("X")); // force libstd itself to make an allocation
        assert!(HITS > start + 1);
    }
}
