rust
#[repr(C, packed(1))]
struct X {
    c: u8,
    m: std::arch::x86_64::__m128,
}

#[no_mangle]
pub fn f() -> usize {
    // Expected 32
    // Actual 17
    std::mem::size_of::<X>()
}
