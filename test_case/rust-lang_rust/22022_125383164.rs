 rust
trait Decoder {
    type Error;
}

trait Decodable {}

fn read_field<D, T>(d: &mut D, field: &str) -> Result<T, D::Error> where
    D: Decoder,
    T: Decodable,
{
    unimplemented!()
}

fn main() {}
