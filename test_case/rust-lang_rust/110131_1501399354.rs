rust
struct SeqBuffer<'a, T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    iter: <&'a T as IntoIterator>::IntoIter,
}

struct Helper<'a, T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    buf: SeqBuffer<'a, T>,
}

impl<'a, T> Helper<'a, T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    fn new(sq: &'a T) -> Self {
        loop {}
    }
}

struct BitReaderWrapper<T>(T);

impl<'a, T> IntoIterator for &'a BitReaderWrapper<T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    type Item = u32;

    type IntoIter = Helper<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Helper::new(&self.0)
    }
}
