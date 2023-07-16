
pub struct ReadBuffer<R> {
    priv reader: R,
    priv buffer: ~[u8],
    priv lo: uint,
    priv hi: uint,
}

impl<R> Decorator<R> for ReadBuffer<R> {
    fn inner(self) -> R { self.reader }
    fn inner_ref<'a>(&'a self) -> &'a R { &self.reader }
    fn inner_mut_ref<'a>(&'a mut self) -> &'a mut R { &mut self.reader }
}

impl<W: Writer> Writer for ReadBuffer<W> {
    fn write(&mut self, buf: &[u8]) {
        self.inner_mut_ref().write(buf)
    }
    fn flush(&mut self) {
        self.inner_mut_ref().flush()
    }
}
