
// other.rs

#[link(name = "other", package_id = "other", vers="0.0")];
#[crate_type = "lib"];

pub enum Value<'v> {
    A(&'v str),
    B,
}

pub trait Decoder<'v> {
    fn read(&mut self) -> Value<'v>;
}

pub trait Decodable<'v, D: Decoder<'v>> {
    fn decode(d: &mut D) -> Self;
}

