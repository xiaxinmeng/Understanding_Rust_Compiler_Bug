
trait Decode {
    type Error;
    fn decode(buf: &[u8]) -> Result<Self, Self::Error>;
}

impl Decode for ! {
    type Error = !;
    fn decode(buf: &[u8]) -> Result<Self, Self::Error> {
        panic!("! can't be decoded")
    }
}
