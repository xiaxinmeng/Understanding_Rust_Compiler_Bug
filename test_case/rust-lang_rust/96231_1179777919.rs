rust
pub fn foo<'a>(n: [*const u8; 2]) -> [&'a u8; 2] {
    n.map(|p| unsafe { &*p })
}
