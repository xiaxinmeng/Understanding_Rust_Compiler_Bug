rust
pub fn oof() -> &'static u8 {
    let refs: [&'static u8; 2] = unsafe { core::mem::uninitialized() };
    refs[0]
}
