rust
fn zero(items: &mut [u8]) {
    for i in 0..32.min(items.len()) {
        items[i] = 0;
    }
    if bounds check failure { report bounds check error }
}
