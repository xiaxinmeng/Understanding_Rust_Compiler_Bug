rust
pub fn sub2(x :&u8) -> usize {
    unsafe {(*x as usize).unchecked_sub(10)}
}
