rust
#![feature(generic_const_exprs)]

use std::mem::size_of;

trait SizedBytes: Sized {
    const BYTES: usize = size_of::<Self>();
}

trait FromBigEndian: SizedBytes {
    fn from_be_bytes(bytes: [u8; Self::BYTES]) -> Self;
}

impl SizedBytes for u64 {}
impl FromBigEndian for u64 {}
