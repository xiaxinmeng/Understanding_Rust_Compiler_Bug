rust
fn foo<const N: usize>() {
    let _: [u8; { 2*N }] = ...;
}
