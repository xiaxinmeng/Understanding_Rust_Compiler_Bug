
impl Writer for SeekableMemWriter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        if self.pos == self.buf.len() {
            self.buf.push_all(buf)
        } else {
            // Make sure the internal buffer is as least as big as where we
            // currently are
            let difference = self.pos as i64 - self.buf.len() as i64;
            if difference > 0 {
                self.buf.grow(difference as uint, &0);
            }

            // Figure out what bytes will be used to overwrite what's currently
            // there (left), and what will be appended on the end (right)
            let cap = self.buf.len() - self.pos;
            let (left, right) = if cap <= buf.len() {
                (buf.slice_to(cap), buf.slice_from(cap))
            } else {
                (buf, &[])
            };

            // Do the necessary writes
            if left.len() > 0 {
                slice::bytes::copy_memory(self.buf.mut_slice_from(self.pos), left);
            }
            if right.len() > 0 {
                self.buf.push_all(right);
            }
        }

        // Bump us forward
        self.pos += buf.len();
        Ok(())
    }
}
