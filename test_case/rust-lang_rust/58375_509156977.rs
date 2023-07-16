rust
pub struct DecodeUtf16<I>(I);

pub trait Arbitrary {
    fn arbitrary() {}
}

pub trait A {
    type Item;
}

impl A for u8 {
    type Item = char;
}

impl Arbitrary for DecodeUtf16<<u8 as A>::Item>  {
}
