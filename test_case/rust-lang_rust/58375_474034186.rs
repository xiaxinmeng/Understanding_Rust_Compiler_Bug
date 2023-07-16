rust
pub struct DecodeUtf16<I: Iterator<Item = u16>>(I);

pub trait Arbitrary {
    type Strategy;
    fn arbitrary_with() -> Self::Strategy;
    fn arbitrary() {}
}

impl Arbitrary for DecodeUtf16<<Vec<u16> as IntoIterator>::IntoIter> {
    type Strategy = Self;
    fn arbitrary_with() -> Self::Strategy {
        unimplemented!()
    }
}
