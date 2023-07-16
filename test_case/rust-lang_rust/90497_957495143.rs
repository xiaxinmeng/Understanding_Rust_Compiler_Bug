rust
#[cfg(target_pointer_width = "32")]
#[lang = "usize"]
impl usize {
    widening_impl! { usize, u64, 32 }
    uint_impl! { usize, u32, isize, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
    "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]",
    usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}

#[cfg(target_pointer_width = "64")]
#[lang = "usize"]
impl usize {
    widening_impl! { usize, u128, 64 }
    uint_impl! { usize, u64, isize, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
    "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
    "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
    usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
}
