rust
let zeroed = MaybeUninit::<(u8, u16)>::zeroed();
ptr::copy::<MaybeUninit<(u8, u16)>>(&zeroed as *const _ as *const _, x, 1);
// does not necessarily overwrite the padding byte
