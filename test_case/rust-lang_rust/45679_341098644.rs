
impl PartialEq for Span {
    fn eq(&self, other: &Span) {
        let a: u32 = self.0;
        let b: u32 = self.1;
        a == b
    }
}
