rs
#![feature(never_type)]
#[allow(unreachable_code)]

pub trait Trait {}
impl Trait for ! {}
impl<T: Trait + ?Sized> Trait for Box<T> {}

pub fn test() -> impl Trait {
    let x: ! = panic!();
    let x: Box<!> = Box::new(x);
    x
}
