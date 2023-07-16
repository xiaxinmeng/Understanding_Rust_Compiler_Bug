rs
#[rustc_must_implement_one_of(read, read_buf)]
pub trait std::io::Read {
    // (new)
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut buf = BorrowedBuf::from(buf);
        self.read_buf(buf.unfilled()).map(|()| buf.len())
    }
    // (existing)
    fn read_buf(&mut self, buf: BorrowedCursor<'_>) -> Result<()> {
        default_read_buf(|b| self.read(b), buf)
    }
}
