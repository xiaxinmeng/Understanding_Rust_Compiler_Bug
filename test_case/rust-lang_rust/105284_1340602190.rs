rust
#![feature(never_type)]

pub trait Trait {}
impl Trait for ! {}
impl<T: Trait + ?Sized> Trait for &T {}

pub fn test() -> impl Trait {
    #[allow(unreachable_code)]
    &panic!()
}
