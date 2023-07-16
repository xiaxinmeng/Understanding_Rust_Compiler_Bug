rs
pub trait FromBytes {
    fn from_bytes(inp: &[u8]) -> Self;
}

pub fn get_from_bytes<T: FromBytes = Vec<u8>>(inp: &[u8]) -> T {
    // ... do something ...
    T::from_bytes(inp)
}
