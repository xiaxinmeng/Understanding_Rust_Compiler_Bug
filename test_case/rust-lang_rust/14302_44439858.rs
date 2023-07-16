
//libserialize/serialize.rs
pub trait Encodable<S:Encoder<E>, E> {
    fn encode(&self, s: &mut S) -> Result<(), E>;
}
