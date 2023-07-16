rust
pub fn foo(n: usize) -> u32 {
    if n > 0 && n < 1001 {
        let a = vec![0_u32; n];
        a[0]
    } else {
        0
    }
}
