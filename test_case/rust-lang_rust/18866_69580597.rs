 rust
use std::io::Reader;

pub struct Foo {
    pub reader: Box<Reader +'static>
}

pub fn bar(reader: &mut Reader) {
}

pub fn new_Foo (reader: Box<Reader +'static>) -> Foo {
    let mut foo = Foo {
        reader: reader
    };
    bar(&mut foo.reader);
    foo
}

fn main() {}
