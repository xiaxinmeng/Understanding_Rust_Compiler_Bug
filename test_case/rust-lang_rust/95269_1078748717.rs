rust
#![feature(allocator_api)]

use core::ptr::NonNull;
use core::alloc::{AllocError, Allocator, Layout};

struct PrintOnDrop;
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("dropped")
    }
}

unsafe impl Allocator for PrintOnDrop {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Err(AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) { }
}

fn main() {
    Vec::<u32, _>::new_in(PrintOnDrop { }).into_iter();
}
