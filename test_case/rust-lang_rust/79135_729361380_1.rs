rust
use generic_array::{GenericArray, ArrayLength};

pub struct Bar<N: ArrayLength<u8>> {
    inner: GenericArray<u8, N>,
}
