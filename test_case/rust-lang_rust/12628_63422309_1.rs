 rust
pub fn unwrap(mut self) -> IoResult<W> {
    try!(self.flush_buf());
    try!(self.inner.take())
}
