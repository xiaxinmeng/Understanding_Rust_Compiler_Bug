
trait Decoder<E> {
    fn read_uint(&self) -> Result<uint, E>;
}

trait Decodable<E, D: Decoder<E>> {
    fn decode(d: &mut D) -> Result<Self, E>;
}

impl<E, D: Decoder<E>> Decodable<E, D> for uint {
    fn decode(d: &mut D) -> Result<uint, E> {
        d.read_uint()
    }
}

trait Encoder<E> {
    fn emit_uint(&self, v: uint) -> Result<(), E>;
}

trait Encodable<E, S: Encoder<E>> {
    fn encode(&self, s: &mut S) -> Result<(), E>;
}

impl<E, S: Encoder<E>> Encodable<E, S> for uint {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_uint(*self)
    }
}
