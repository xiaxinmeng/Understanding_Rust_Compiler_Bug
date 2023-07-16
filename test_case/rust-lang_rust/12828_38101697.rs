 rust
pub struct Encoder<'a> {
    priv wr: &'a mut io::Writer,
}
