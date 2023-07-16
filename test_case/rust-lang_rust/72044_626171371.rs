
error: cannot specialize on trait `serialize::Encodable`
   --> src/libserialize/serialize.rs:644:1
    |
644 | / impl<T: ?Sized + Encodable> Encodable for Box<T> {
645 | |     fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
646 | |         (**self).encode(s)
647 | |     }
648 | | }
    | |_^

error: cannot specialize on trait `serialize::Decodable`
   --> src/libserialize/serialize.rs:650:1
    |
650 | / impl<T: Decodable> Decodable for Box<T> {
651 | |     fn decode<D: Decoder>(d: &mut D) -> Result<Box<T>, D::Error> {
652 | |         Ok(box Decodable::decode(d)?)
653 | |     }
654 | | }
    | |_^
