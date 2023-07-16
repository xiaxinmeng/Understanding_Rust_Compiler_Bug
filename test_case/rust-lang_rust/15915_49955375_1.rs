 rust
#[inline]                                    
fn write(&mut self, buf: &[u8]) -> io::IoResult<()> {
    if self.pos == uint::MAX {                       
        self.buf.push_all(buf);                      
        return Ok(())                                
    }                                                
    self.write_slow(buf)
}          

#[inline(never)] fn write_slow(...) { ... }       
