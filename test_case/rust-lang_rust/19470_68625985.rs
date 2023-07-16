 Rust

trait UniversalDecodable {
    fn decode_by<D: Decoder<E>, E>(d: &mut D) -> Result<Self, E>;
}

impl<D: Decoder<E>, E, T: UniversalDecodable> Decodable<D, E> for T {
    fn decode(d: &mut D) -> Result<Self, E> {
        UniversalDecodable::decode_by(d)
    }
}
