rust
impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let handle = get_handle(c::STD_INPUT_HANDLE)?;
        if !is_console(handle) {
            unsafe {
                let handle = Handle::from_raw_handle(handle);
                let ret = handle.read(buf);
                handle.into_raw_handle(); // Don't close the handle
                return ret;
            }
        }
        // If there are bytes in the incomplete utf-8, start with those.
        // (No-op if there is nothing in the buffer.)
        let mut bytes_copied = self.incomplete_utf8.read(buf);

        if bytes_copied == buf.len() {
            return Ok(bytes_copied);
        } else if buf.len() - bytes_copied < 4 {
            // Not enough space to get a UTF-8 byte. We will use the incomplete UTF8.
            let mut utf16_buf = [0u16; 1];
            // Read one u16 character.
            let read = read_u16s_fixup_surrogates(handle, &mut utf16_buf, 1, &mut self.surrogate)?;
            // Read bytes, using the (now-empty) self.incomplete_utf8 as extra space.
            let read_bytes = utf16_to_utf8(&utf16_buf[..read], &mut self.incomplete_utf8.bytes)?;

            // Read in the bytes from incomplete_utf8 until the buffer is full.
            self.incomplete_utf8.len = read_bytes as u8;
            // No-op if no bytes.
            bytes_copied += self.incomplete_utf8.read(&mut buf[bytes_copied..]);
            Ok(bytes_copied)
        } else {
            let mut utf16_buf = [0u16; MAX_BUFFER_SIZE / 2];
            // In the worst case, a UTF-8 string can take 3 bytes for every `u16` of a UTF-16. So
            // we can read at most a third of `buf.len()` chars and uphold the guarantee no data gets
            // lost.
            let amount = cmp::min(buf.len() / 3, utf16_buf.len());
            let read =
                read_u16s_fixup_surrogates(handle, &mut utf16_buf, amount, &mut self.surrogate)?;

            match utf16_to_utf8(&utf16_buf[..read], buf) {
                Ok(value) => return Ok(bytes_copied + value),
                Err(e) => return Err(e),
            }
        }
    }
}
