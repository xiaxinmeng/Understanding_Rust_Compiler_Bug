rust
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Foo {
    Zero,
    One,
    Two,
}

pub fn compare(a: Foo, b: Foo)->bool{
    a <= b
}
