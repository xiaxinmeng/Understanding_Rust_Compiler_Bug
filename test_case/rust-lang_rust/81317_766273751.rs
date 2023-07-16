rust
use std::ops::{BitXor, Shl};

pub trait Protocol {
    type PacketIndex: Into<u64> + Into<BigUint>;
}

fn from_nothing<T>() -> T {
    todo!()
}

pub fn decrypt_portion<P: Protocol>(index: P::PacketIndex) {
    let iv: BigUint = from_nothing();
    let len: usize = from_nothing();
    let iv = iv ^ (index.into() << 16);
    let iv = iv ^ (BigUint::from(1_u8) << (len * 8));
    let _iv: &[u8] = &iv.to_bytes_be()[1..len + 1];
}

pub struct BigUint;

impl BigUint {
    fn to_bytes_be(&self) -> Vec<u8> {
        todo!()
    }
}

impl From<u8> for BigUint {
    fn from(_: u8) -> Self {
        unimplemented!()
    }
}

impl Shl<usize> for BigUint {
    type Output = BigUint;

    fn shl(self, _rhs: usize) -> Self::Output {
        unimplemented!()
    }
}

impl BitXor for BigUint {
    type Output = BigUint;

    fn bitxor(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<'a> BitXor<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn bitxor(self, _rhs: &'a BigUint) -> Self::Output {
        unimplemented!()
    }
}

