rust
pub union BagOfBits<T: Copy> {
    uninit: (),
    _storage: T,
}

pub fn foo(x: BagOfBits<usize>) {}
