rust
assert_eq!(
    // as_ptr INside the `const{}`
    (const { (unsafe { std::slice::from_raw_parts(3 as *const u8, 0) }).as_ptr() }),
    std::ptr::invalid(3),
); // PASSES
assert_eq!(
    // as_ptr OUTside the `const{}`
    (const { (unsafe { std::slice::from_raw_parts(7 as *const u8, 0) }) }).as_ptr(),
    std::ptr::invalid(7),
); // FAILS, 0x56229d3aa00b != 0x7
