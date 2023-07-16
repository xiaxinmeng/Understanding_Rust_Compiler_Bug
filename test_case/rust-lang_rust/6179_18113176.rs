 rust
pub struct IString(uint);

impl Eq for IString {
    fn eq(&self, other: &IString) -> bool { *self == *other }
    fn ne(&self, other: &IString) -> bool { *self != *other }
}

pub fn foo(a: IString, b: IString) -> bool { a==b }
