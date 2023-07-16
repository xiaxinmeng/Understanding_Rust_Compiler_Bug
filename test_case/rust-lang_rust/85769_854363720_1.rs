rust
pub const unsafe fn random() -> usize {
    let a = 5;
    std::mem::transmute(&a)
}
