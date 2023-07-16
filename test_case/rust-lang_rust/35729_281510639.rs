rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Slice<'a> {
    bytes: &'a [u8]
}

impl<'a> Slice<'a> {
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Slice<'a> {
        Slice { bytes: bytes }
    }

    #[inline]
    pub fn get<I>(&self, i: I) -> Option<&I::Output>
                  where I: std::slice::SliceIndex<u8> {
        self.bytes.get(i)
    }
}
