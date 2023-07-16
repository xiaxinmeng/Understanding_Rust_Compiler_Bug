rust
// src/main.rs
#![no_std]
#![no_main]

extern crate alloc;

#[link(name = "c")]
extern "C" {}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }

struct A;
unsafe impl alloc::alloc::GlobalAlloc for A {
unsafe fn alloc(&self, _: alloc::alloc::Layout) -> *mut u8 { todo!() }
unsafe fn dealloc(&self, _: *mut u8, _: alloc::alloc::Layout) { todo!() }
}

#[global_allocator]
static ALLOC: A = A;

#[no_mangle]
extern "C" fn main() {
    alloc::alloc::handle_alloc_error(alloc::alloc::Layout::new::<()>());
}
