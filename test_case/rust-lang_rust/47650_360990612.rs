rust
union U {
    a: [u8],
    b: [u16],
}
=>
struct U::Meta {
    a: usize,
    b: usize,
}
