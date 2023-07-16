 rust
pub trait Buffer: Reader {
    fn fill_buf<'a>(&'a mut self) -> IoResult<&'a [u8]>;
    // ...
}
