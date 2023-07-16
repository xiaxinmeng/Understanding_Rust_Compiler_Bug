rust
#![feature(arbitrary_enum_discriminant)]

#[repr(u8)]
enum Enum {
    Foo = 1,
    Bar(),
    Baz{}
}

fn main() { }
