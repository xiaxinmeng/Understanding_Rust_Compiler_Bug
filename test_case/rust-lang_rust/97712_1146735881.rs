rust
let mut x = 5u8;
let mut y = 6u8;

unsafe {
    ptr::swap(&mut x as *mut u8 as *mut bool, &mut y as *mut u8 as *mut bool)
}
// and similarly, swap_nonoverlapping, copy, copy_nonoverlapping
