rust
pub fn foo(a: &mut [u8]) {
    for i in 0..32 {
        a[i] = 1;
    }
}
