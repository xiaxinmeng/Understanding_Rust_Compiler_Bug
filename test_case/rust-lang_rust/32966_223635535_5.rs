 rust
fn foo() -> [u8; 1024] {
    let x = [0; 1024];
    x // will interact w/ debuginfo
}
