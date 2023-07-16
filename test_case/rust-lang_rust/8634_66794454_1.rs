 rust
pub trait DuktapeEncodable: Encodable<Encoder, DuktapeError> {}
impl<T: Encodable<Encoder, DuktapeError>> DuktapeEncodable for T {}
