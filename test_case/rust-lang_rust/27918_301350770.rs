rust
fn foo() {
    let a: &[u8] = &[];
    const B: &'static [u8] = b"";
    match (a, 0) {
        (B, 0)  => {},
        _ => {},
    }
}
