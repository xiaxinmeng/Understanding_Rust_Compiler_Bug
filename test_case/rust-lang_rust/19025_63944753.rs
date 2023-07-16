
fn skip_exact(&mut self, n: uint) -> Result<()> {
    if n < THRESHOLD {
        let buf: [u8, ..THRESHOLD] = unsafe { mem::uninitialized() };
        try!(self.read_at_least(n, buf[..n]));
    }
    else {
        try!(self.read_exact(n));
    }
    Ok(())
}
