
use std::io;

pub struct Container<'a> {
    reader: &'a mut Reader
}

impl<'a> Container<'a> {
    pub fn wrap<'s>(reader: &'s mut Reader) -> Container<'s> {
        Container { reader: reader }
    }

    pub fn read_to(&self, vec: &mut [u8]) {
        self.reader.read(vec);
    }
}

pub fn for_stdin<'a>() -> Container<'a> {
    let mut r = io::stdin();
    Container::wrap(&mut r as &mut Reader)
}
