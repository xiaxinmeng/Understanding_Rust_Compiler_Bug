rust
pub struct Foo<const N: u8>;

pub type Bar = Foo<{ *(0u8..=0x45).end() }>;

fn main() {}
