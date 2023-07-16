 rust
impl<'a> Hasher for Sha256Hasher<'a> {
    fn write_usize(&mut self, i: usize) { bug!("can't do that here") }
    fn write_isize(&mut self, i: isize) { bug!("can't do that here") }
}
