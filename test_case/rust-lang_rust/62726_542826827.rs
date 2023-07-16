rust
impl Buf for IoSlice<'_> {
    fn advance(&mut self, cnt: usize) {
        *self = IoSlice::new(&self[cnt..]); // conflicting lifetime requirements
    }
}
