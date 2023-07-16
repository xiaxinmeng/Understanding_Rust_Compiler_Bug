rust
let zeroed = [0u16; 2];
ptr::copy::<MaybeUninit<(u8, u16)>>(&zeroed as *const _ as *const _, x, 1);
// writes all four bytes
