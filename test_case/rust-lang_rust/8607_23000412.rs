
use std::rt::io::{Reader, ReaderByteConversions};

struct Foo;

impl Reader for Foo {
    fn read(&mut self, _buf: &mut [u8]) -> Option<uint> { None }
    fn eof(&mut self) -> bool { true }
}

fn main() {
    let mut f = Foo; 
    f.read_u8();
}
