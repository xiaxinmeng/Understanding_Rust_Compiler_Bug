rust
pub fn truncate(&mut self, len: usize) {
    // This is safe because:
    //
    // * the slice passed to `drop_in_place` is valid; the `len > self.len` case avoids
    //   creating an invalid slice, and
    // * the `len` of the vector is shrunk before calling `drop_in_place`, such that no value
    //   will be dropped twice in case `drop_in_place` were to panic once (if it panics twice,
    //   the program aborts).
    unsafe {
        if len > self.len() {
            return;
        }
        let remaining_len = self.len() - len;
        let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
        self.buf.set_len(len);
        ptr::drop_in_place(s);
    }
}
