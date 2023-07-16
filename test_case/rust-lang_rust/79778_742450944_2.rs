rust
fn foo<const N: usize>() {
    let x: [u8; N / 8] = ...;
}
