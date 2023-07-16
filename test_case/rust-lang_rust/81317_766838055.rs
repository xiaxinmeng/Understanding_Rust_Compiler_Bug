rust
use std::ops::BitXor;

pub struct S;

pub trait P {
    type I: Into<u64> + Into<S>;
}

pub fn decrypt_portion<T: P>(index: T::I) {
    let iv = S ^ index.into();
    &iv.to_bytes_be();
}

impl S {
    fn to_bytes_be(&self) -> &[u8] {
        unimplemented!()
    }
}

impl BitXor for S {
    type Output = S;

    fn bitxor(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<'a> BitXor<&'a S> for S {
    type Output = S;

    fn bitxor(self, _rhs: &'a S) -> Self::Output {
        unimplemented!()
    }
}
