rust
// this generates a panic branch
pub enum All {
    None = 0,
    Foo = 1,
    Bar = 2,
}

// these do not
#[repr(u8)]
pub enum All {
    None,
    Foo,
    Bar,
}
pub enum All {
    None = 1,
    Foo = 2,
    Bar = 3,
}
pub enum All {
    None,
    Foo,
    Bar = 3,
}
pub enum All {
    None = -1,
    Foo,
    Bar,
}
