rust
pub fn foo() -> u8 {
    let mut v = Vec::<u8>::with_capacity(1);
    v.push(1);
    return v[0];
}
