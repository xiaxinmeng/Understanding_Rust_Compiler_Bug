rust
// compile-flags: -O
// min-llvm-version: 10.0

#![crate_type = "lib"]

#[repr(u8)]
pub enum Exception {
    Low = 5,
    High = 10,
}

// CHECK-LABEL: @access
#[no_mangle]
pub fn access(array: &[usize; 12], exc: Exception) -> usize {
    // CHECK-NOT: panic_bounds_check
    array[(exc as u8 - 4) as usize]
}
