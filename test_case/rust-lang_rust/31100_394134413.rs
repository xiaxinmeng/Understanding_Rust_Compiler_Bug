rust
fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let result: u64;
        if let SeekFrom::Current(n) = pos {
            if let Some(new_pos) = (self.pos as i64).checked_add(n) {
                if new_pos >= 0 && new_pos <= self.cap {
                    self.pos = new_pos as usize;
                    return self.inner.seek(SeekFrom::Current(0)).map(|p| p + (self.cap - self.pos));
                }
            }
            // .. the rest is the same as before
