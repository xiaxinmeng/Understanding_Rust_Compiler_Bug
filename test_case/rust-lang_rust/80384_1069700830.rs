rust
#![feature(const_refs_to_cell)]

use core::cell::UnsafeCell;
use core::ptr;

fn main() {
    const _: i32 = {
        let x = UnsafeCell::new(5i32);
        let p: *const UnsafeCell<i32> = ptr::addr_of!(x);

        unsafe {
            // No stable way to mutate `p`

            // Requires const_mut_refs
            // *(p as *mut i32) = 10

            // Requires const_ptr_write
            // (p as *mut i32).write(10)

            // Requires const_intrinsic_copy, which just left FCP
            // ptr::copy(&10, p as *mut i32, 1);

            *(p as *const i32)
        }
    };
}
