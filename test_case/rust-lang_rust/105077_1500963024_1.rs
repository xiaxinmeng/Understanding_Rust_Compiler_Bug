rust
#[restrict(mut(crate))] // applies to all fields in struct
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
}

pub enum Foo {
    #[restrict(mut(crate))] // applies to all fields in variant
    Alpha { x1: u8, x2: u8 },
    Beta { y: u8 },
}

#[restrict(mut(crate))] // applies to all fields in all variants in enum
pub enum Bar {
    Alpha { x1: u8, x2: u8 },
    Beta { y: u8 },
}
