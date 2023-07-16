rust
    pub fn write_vectored(&self, buf: &[IoVec<'_>]) -> io::Result<usize> {
        let buf = match buf.get(0) {
            Some(buf) => buf,
            None => return Ok(0),
        };
        self.write(buf)
    }
