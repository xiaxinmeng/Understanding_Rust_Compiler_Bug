rust
let arr: [u8; 4] = [1, 2, 3, 4];
let arr_ptr: *const u8 = arr.as_ptr();
unsafe {
    let u32_ptr: *const u32 = core::mem::transmute::<_, *const u32>(arr_ptr);
    let u32_val = u32_ptr.read_unaligned();
    assert_eq!(u32_val.to_ne_bytes(), arr);
}
