
// main.rs

extern mod other;

struct S;

impl<'v, D: other::Decoder<'v>> other::Decodable<'v, D> for S {
    fn decode(d: &mut D) -> S {
        match d.read() {
            other::A(..) => S,
            other::B => other::Decodable::decode(d),
        }
    }
}

fn main (){}
