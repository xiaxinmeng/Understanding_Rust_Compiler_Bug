 rust
impl Display for char {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.width.is_none() && f.precision.is_none() {
            f.write_char(*self)
        } else {
            let mut utf8 = [0; 4];
            let amt = self.encode_utf8(&mut utf8).unwrap_or(0);
            let s: &str = unsafe { mem::transmute(&utf8[..amt]) };
            f.pad(s)
        }
    }
}
