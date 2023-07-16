
use std::rt::io::{Writer, WriterByteConversions};

struct Foo;

impl Writer for Foo { 
    fn write(&mut self, _buf: &[u8]) {}
    fn flush(&mut self) {}
} 

fn main() {
    let mut f = Foo; 
    f.write_le_i32(0);
}
