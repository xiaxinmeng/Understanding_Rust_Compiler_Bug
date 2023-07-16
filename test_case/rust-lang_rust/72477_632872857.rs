rust
pub struct Foo<'a> {
    buf: &'a mut [u8],
}

impl<'a> Foo<'a> {
    fn foo(mut self: std::pin::Pin<&mut Self>) {
        let (_, rest) = std::mem::replace(&mut self.buf, &mut []).split_at_mut(1);
        self.buf = rest;
    }
}
