rust
pub static FOO: usize = unsafe {
    let r: &char = &'x';
    let r2i: usize = std::mem::transmute(r);
    r2i + 0
};
