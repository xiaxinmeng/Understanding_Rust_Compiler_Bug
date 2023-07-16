rust
use std::{mem, ptr};

fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        // Give ourselves some scratch space to work with
        let mut t: [u8; 16] = mem::uninitialized();

        let x = x as *mut T as *mut u8;
        let y = y as *mut T as *mut u8;
        let t = &mut t as *mut _ as *mut u8;

        // can't use a for loop as the `range` impl calls `mem::swap` recursively
        let len = mem::size_of::<T>() as isize;
        let mut i = 0;
        while i + 16 <= len {
            // Perform the swap 16 bytes at a time, `&mut` pointers never alias
            ptr::copy_nonoverlapping(x.offset(i), t, 16);
            ptr::copy_nonoverlapping(y.offset(i), x.offset(i), 16);
            ptr::copy_nonoverlapping(t, y.offset(i), 16);
            i += 16;
        }
        if i < len {
            // Swap any remaining bytes
            let rem = (len - i) as usize;
            ptr::copy_nonoverlapping(x.offset(i), t, rem);
            ptr::copy_nonoverlapping(y.offset(i), x.offset(i), rem);
            ptr::copy_nonoverlapping(t, y.offset(i), rem);
        }
    }
}

#[no_mangle]
pub fn swap_array(x: &mut [u8; 2048], y: &mut [u8; 2048]) {
    swap(x, y)
}
