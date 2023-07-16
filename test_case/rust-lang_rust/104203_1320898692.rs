rust
const U16_SLICE: &[u16] = {
    let u8_slice: &[u8] = &[1, 2, 3, 4];
    let u16_slice = unsafe { core::slice::from_raw_parts(u8_slice.as_ptr().cast::<u16>(), 2) };

    // ... lots of other code here ...

    u16_slice
};
