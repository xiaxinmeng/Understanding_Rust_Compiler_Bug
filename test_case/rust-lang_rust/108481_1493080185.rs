rust
use std::fmt::Debug;

#[derive(Debug)] // I am not able to expand this derive.
pub struct ConstGeneric<const CHUNK_SIZE: usize> {
    _p: [(); CHUNK_SIZE],
}

/////// MOVE START
impl<const CHUNK_SIZE: usize> ConstGeneric<CHUNK_SIZE> {}
/////// MOVE END
