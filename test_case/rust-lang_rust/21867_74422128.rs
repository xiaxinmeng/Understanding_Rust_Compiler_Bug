 rust
extern crate serialize;
use serialize::{Decoder, Decodable};

struct A;

impl Decodable for A {
    fn decode<D>(s: &mut D) -> Result<Self, Decoder::Error>
        where D: Decoder {
    }
}

fn main() {}
