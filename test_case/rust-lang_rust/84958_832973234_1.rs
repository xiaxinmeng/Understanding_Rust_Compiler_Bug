rust
#![no_std]

#[derive(Clone, Copy)]
pub struct Slice<'a> {
    pub bytes: &'a [u8],
}
impl<'a> Slice<'a> {
    #[inline]
    pub fn subslice(&self, r: core::ops::Range<usize>) -> Self {
        let bytes = self.bytes.get(r).unwrap();
        Self { bytes }
    }
    pub fn into_value(self) -> Slice<'a> {
        self
    }

    pub fn read_all<F, R>(self, read: F) -> R
    where
        F: FnOnce(&mut Reader<'a>) -> R,
    {
        let mut input = Reader::new(self.into_value());
        read(&mut input)
    }
}
pub struct Reader<'a> {
    input: Slice<'a>,
    i: usize,
}
impl<'a> Reader<'a> {
    #[inline]
    pub fn new(input: Slice<'a>) -> Self {
        Self {
            input: input.into_value(),
            i: 0,
        }
    }
    #[inline]
    pub fn at_end(&self) -> bool {
        self.i == self.input.bytes.len()
    }
    #[inline]
    pub fn read_byte(&mut self) -> u8 {
        let b = self.input.bytes.get(self.i).unwrap();
        self.i += 1;
        *b
    }
    #[inline]
    pub fn read_bytes(&mut self, num_bytes: usize) -> Slice<'a> {
        let new_i = self.i.checked_add(num_bytes).unwrap();
        let bytes = self.input.subslice(self.i..new_i);
        self.i = new_i;
        bytes
    }
}
