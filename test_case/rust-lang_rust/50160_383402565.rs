rust
impl<'a, T> Cow<'a, T> {
    pub fn borrowed<'b>(&'b self) -> Cow<'b, T> where 'a: 'b {
        Cow::Borrowed(self.o.borrow())
    }
}
