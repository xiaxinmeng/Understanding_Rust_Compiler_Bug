
pub enum Value<'a> {
    A(&'a str),
    B,
    C,
}

pub trait Decoder {
    fn read<'a>(&'a mut self) -> Value<'a>;
}

pub trait Decodable<D: Decoder> {
    fn decode(d: &mut D) -> Self;
}

impl<D: Decoder> Decodable<D> for () {
    fn decode(d: &mut D) -> () {
        match d.read() {
            Value::A(x) => { let _ = x; () }
            Value::B => Decodable::decode(d),
            Value::C => Decodable::decode(d),
        }
    }
}

fn main() {}
