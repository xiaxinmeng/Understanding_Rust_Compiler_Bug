rust
#![feature(const_generics)]

trait Tr {
    const C: usize = 0;
    fn fun(x: [u8; Self::C]) -> [u8; 0] { x }
}
