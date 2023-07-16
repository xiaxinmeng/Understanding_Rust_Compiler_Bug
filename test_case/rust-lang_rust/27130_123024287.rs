 rust
pub fn trim_in_place(a: &mut &[u8]) {
    let mut x = *a;
    while x.first() == Some(&42) {
        x = &x[1..];
    }
    *a = x;
}
