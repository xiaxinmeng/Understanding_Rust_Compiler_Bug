rust
pub fn foo(x: usize) -> u32 {
    let base: [u32; 5] = *&[1,3,4,5,2];
    base[x % 5]
}
