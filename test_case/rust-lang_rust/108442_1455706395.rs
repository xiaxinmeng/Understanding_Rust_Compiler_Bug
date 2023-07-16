rust
pub enum Void {}

#[inline]
pub unsafe fn unreachable() -> ! {
    let x: &Void = std::mem::transmute(1usize);
    match *x {}
}
