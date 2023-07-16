 rust
impl<'a> Writer for &'a mut Writer+'a {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> IoResult<()> { self.write(buf) }

    #[inline]
    fn flush(&mut self) -> IoResult<()> { self.flush() }
}
