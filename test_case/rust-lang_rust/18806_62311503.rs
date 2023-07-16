 rust
pub struct Foo {
    header: u16,
    bytes: [u8]
}

impl Foo {
    fn new(bytes: Box<[u8]>) -> Box<Foo> {
        box Foo { header: 4, bytes: *bytes }
    }
}
