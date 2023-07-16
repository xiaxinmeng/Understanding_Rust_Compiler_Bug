 rust
impl fmt::Show for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "os error {}"));
        match self.detail { Some(ref s) => try!(write!(f, " ({})", s)); None => {} }
        Ok(())
    }
}
