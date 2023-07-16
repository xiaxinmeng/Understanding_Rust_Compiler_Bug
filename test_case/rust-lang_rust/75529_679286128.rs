rust
// compile-flags: -O
// min-llvm-version: 10.0

#![crate_type = "lib"]

pub enum Bar {
    A = 1,
    B = 2
}

// CHECK-LABEL: @lookup_inc
#[no_mangle]
pub fn lookup_inc(buf: &[u8; 5], f: Bar) -> u8 {
    // CHECK-NOT: panic_bounds_check
    buf[f as usize + 1]
}

// CHECK-LABEL: @lookup_dec
#[no_mangle]
pub fn lookup_dec(buf: &[u8; 5], f: Bar) -> u8 {
    // CHECK-NOT: panic_bounds_check
    buf[f as usize - 1]
}
