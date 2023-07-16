 rust
use std::io::Read;

pub struct Foo {
    pub reader: Box<Read +'static>
}

pub fn bar(reader: &mut Read) {
}

pub fn new_Foo (reader: Box<Read +'static>) -> Foo {
    let mut foo = Foo {
        reader: reader
    };
    bar(&mut foo.reader);
    foo
}

fn main() {}
