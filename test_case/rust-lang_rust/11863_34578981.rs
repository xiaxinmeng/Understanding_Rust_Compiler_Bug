
trait Hasher {
    fn hash_u8(x: u8);
    fn hash_u16(x: u16);
    ...
}

trait Hash<H: Hasher> {
    fn hash(&self, &mut H);
}
