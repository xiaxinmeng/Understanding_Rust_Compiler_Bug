 rust
#![feature(alloc, heap_api)]

extern crate alloc;

fn main() {
    let size = std::mem::size_of::<()>();
    let align = std::mem::align_of::<()>();

    unsafe {
        alloc::heap::allocate(size, align);
    }
}
