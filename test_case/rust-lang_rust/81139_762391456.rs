rust
trait Read {
    /* Other methods */

    // TODO: Find a better name?
    fn read_fill(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        let mut written = 0;
        loop {
            let len = match self.read(buf) {
                Ok(0) => return Ok(written),
                Ok(len) => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            written += len;
            buf = &mut buf[len..];
        }
    }
}
