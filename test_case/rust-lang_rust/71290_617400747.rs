rust
pub fn foo(a: bool, b: bool) -> u8 {
    ({ u8::from(a) } + { u8::from(b) })
}
