rust
pub static FOO: u8 = unsafe {
    let r: &char = &'x';
    let r2i: usize = std::mem::transmute(r);
    let r2i2r: &u8 = std::mem::transmute(r2i);
    *r2i2r
};
