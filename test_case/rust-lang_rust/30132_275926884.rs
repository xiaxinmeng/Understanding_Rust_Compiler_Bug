rust
use std::marker::PhantomData;

struct Cursor<T>(PhantomData<T>);

impl<T> Cursor<T> {
    fn new(_: T) -> Self {
        Cursor(PhantomData)
    }
}

trait Write {
    fn flush(&mut self) {}
}

impl<'a> Write for Cursor<&'a mut [u8]> {}
impl Write for Cursor<Vec<u8>> {}

// This code doesn't type check if this is uncommented:
// impl<'a> Write for Cursor<&'a mut Vec<u8>> {}

fn main() {
    let mut vec = vec![0u8];
    Cursor::new(vec.as_mut()).flush();
}
