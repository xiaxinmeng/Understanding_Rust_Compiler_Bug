
#![feature(type_alias_enum_variants)]
pub enum Enum {
    A(usize),
}

impl Enum {
    fn foo(&self) -> () {
        match self {
            Self::A => (),
        }
    }
}

fn main() {}
