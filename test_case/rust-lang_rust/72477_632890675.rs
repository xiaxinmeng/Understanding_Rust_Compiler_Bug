rust
pub struct Foo<'a> {
    buf: &'a mut [u8],
}

impl<'a> Foo<'a> {
    fn repro(&mut self) { 
        self.buf = &mut self.buf[1..]; //~ ERROR
    }

    fn fixed(&mut self) {
        let (_, rest) = std::mem::replace(&mut self.buf, &mut []).split_at_mut(1);
        self.buf = rest;
    }
}
