 rust
impl<'a,T: Buffer+'a> Buffer for ChunkBuffer<'a,T> {
    fn fill_buf<'a>(&'a mut self) -> IoResult<&'a [u8]> {
        // ...

            { // Block A.
                let read_or_err = self.input.fill_buf();
                match read_or_err {
                    Err(err) => { return Err(err); }
                    Ok(read) => {
                        if read.contains_slice(self.boundary.as_slice()) {
                               return Ok(unsafe { transmute(read) });
                        }
                    }
                }
            }
            self.top_up()
