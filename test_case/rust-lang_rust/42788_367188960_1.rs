rust
impl<T: Read> ReadBuf for T {
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) {
        self.read(buf.zeroed())
    }
}

impl<T: ReadUninit> ReadBuf for T {
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) {
        self.read(buf.bytes())
    }
}
