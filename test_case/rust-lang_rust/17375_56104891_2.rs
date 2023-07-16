 rust
struct Foo {
    name: Vec<u8>
}

impl Foo {
    fn name_as_str<'a>(&'a self) -> Option<&'a str> {
         str::from_utf8(&(self.as_slice()))
    }
}
