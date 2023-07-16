rust
fn zero(items: &mut [u8]) {
    for i in 0..32 {
        items[i] = 0;
    }
}
