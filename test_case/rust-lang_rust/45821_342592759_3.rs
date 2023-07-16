rust
#[no_mangle]
pub fn two_valued(x: Two) -> Two {
    unsafe {
        std::intrinsics::assume(2 > x as u8);
    }
    match x {
        First => First,
        Second => Second,
    }
}
