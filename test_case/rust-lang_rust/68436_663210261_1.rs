rust
fn foo<const C: u8>() {
    if C > 0 {
        let _: [u8; 25 / C];
    } else {
        let _: [u8; 0];
    }
}
