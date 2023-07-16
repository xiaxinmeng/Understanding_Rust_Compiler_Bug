
pub enum Value<'self> {
    A(&'self str),
    B,
}

pub trait Decoder<'self> {
    fn read(&mut self) -> Value<'self>;
}

pub trait Decodable<'self, D: Decoder<'self>> {
    fn decode(d: &mut D) -> Self;
}

pub fn decode<
    'a,
    D: Decoder<'a>,
    T: Decodable<'a, D>
>(d: &mut D) -> Option<T> {
    Decodable::decode(d)
}

impl<'self, D: Decoder<'self>> Decodable<'self, D> for () {
    fn decode(d: &mut D) -> () {
        match d.read() {
            A(*) => (),
            B => Decodable::decode(d),
        }
    }
}

fn main() { }
