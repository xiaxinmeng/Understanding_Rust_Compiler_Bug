rust
pub fn sub3(x :&u8) -> usize {
    unsafe {(*x).unchecked_sub(10) as usize}
}
