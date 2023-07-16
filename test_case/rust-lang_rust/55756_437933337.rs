rust
#![crate_type="lib"]

pub trait Database<'a> {
    type Guard: 'a;
}

pub struct Stateful<'a, D: 'a>(&'a D);

impl<'b, D: for <'a> Database<'a>> Stateful<'b, D> {
    pub fn callee<'a>(&'a self) -> <D as Database<'a>>::Guard {
        unimplemented!()
    }
    pub fn caller<'a>(&'a self) -> <D as Database<'a>>::Guard {
        let state = self.callee();
        unimplemented!()
    }
}
