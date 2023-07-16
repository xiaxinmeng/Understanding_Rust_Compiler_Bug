rust
use std::ops::Deref;

/// fooo
pub struct Foo {
    /// lol
    pub x: u32,
    /// yeay
    pub y: i32,
}

impl Deref for Foo {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.x
    }
}
