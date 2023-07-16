rust
use std::ptr;

let mut array = [0, 1, 2, 3];

let x = array[0..].as_mut_ptr() as *mut [u32; 3]; // this is `array[0..3]`

// this as_mut_ptr() invalidates x!
let y = array[1..].as_mut_ptr() as *mut [u32; 3]; // this is `array[1..4]`

unsafe {
    // using invalidated x
    ptr::swap(x, y);

    // (snip)
}
