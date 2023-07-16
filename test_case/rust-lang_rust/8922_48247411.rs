 rust
trait Writer<M> {
    fn write(&mut self, buf: &[u8]) -> M<()>;
}

impl Writer<Identity> for MemWriter {
    fn write(&mut self, buf: &[u8]) -> Identity<()> {
        self.buf.push_all(buf);
        Identity(())
    }
} 

impl FdWriter<IoResult> for FileWriter {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        let len = libc::write(self.fd, buf.as_ptr());
        if len == -1 {
           Err(io_error_from_errno())
        } else {
           Ok(())
        }
    }
}
