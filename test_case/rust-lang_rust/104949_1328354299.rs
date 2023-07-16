rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct MaxLengthString<const N: u8> 
where [(); N as usize]:
{
    len: u8,
    buf: [u8; N as usize],
}
