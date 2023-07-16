rust
pub fn foo() -> Box<[u8; 8]> {
    Box::new([0; 8])
}

pub fn bar() -> Box<[u8; 9]> {
    Box::new([0; 9])
}
