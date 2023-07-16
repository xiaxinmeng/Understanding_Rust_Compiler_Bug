
use std::ops::Index;
struct Foo;

impl Index<str> for Foo {
    type Output = u8;
    fn index(&self, index: str) -> &u8 {
        1u8
    }
}
