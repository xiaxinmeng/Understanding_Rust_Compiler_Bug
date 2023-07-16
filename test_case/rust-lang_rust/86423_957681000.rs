rust
pub trait BufRead2: Read {
    fn read_buf(&mut self) -> IoResult<Option<&[u8]>>; // a way to express non empty slice ?
    fn consume(&mut self, n: usize) -> Result<???,  ???> ;
}
