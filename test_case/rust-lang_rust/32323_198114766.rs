 rust
pub trait Tr<'a> {
    type Out;
}

pub fn f<'a, T: Tr<'a>>() -> <T as Tr<'a>>::Out {}

pub fn main() {}
