 rust
/// A buffer which breaks chunks only after the specified boundary
/// sequence, or at the end of a file, but nowhere else.
pub struct ChunkBuffer<'a, T: Buffer+'a> {
    input:  &'a mut T,
    boundary: Vec<u8>,
    buffer: Vec<u8>
}

impl<'a, T: Buffer+'a> ChunkBuffer<'a,T> {
    // Called internally to make `buffer` valid.  This is where all our
    // evil magic lives.
    fn top_up<'b>(&'b mut self) -> IoResult<&'b [u8]> {
        // ...
    }
}

impl<'a,T: Buffer+'a> Buffer for ChunkBuffer<'a,T> {
    fn fill_buf<'a>(&'a mut self) -> IoResult<&'a [u8]> {
        if self.buffer.as_slice().contains_slice(self.boundary.as_slice()) {
            // Exit 1: Valid data in our local buffer.
            Ok(self.buffer.as_slice())
        } else if self.buffer.len() > 0 {
            // Exit 2: Add some more data to our local buffer so that it's
            // valid (see invariants for top_up).
            self.top_up()
        } else {
            {
                // Exit 3: Exit on error.
                let read = try!(self.input.fill_buf());
                if read.contains_slice(self.boundary.as_slice()) {
                    // Exit 4: Valid input from self.input. Yay!
                    return Ok(read)
                }
            }
            // Exit 5: Accumulate sufficient data in our local buffer (see
            // invariants for top_up).
            self.top_up()
        }
    }
