rs
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
pub struct Ptr(*const ());

pub trait Trait {
    fn get_translate<const F: Ptr>() {}
}
impl Trait for () {}

pub trait WrapperFunc<const F: Ptr> {
    type CallConv;
}

fn f() {}

pub fn foo() {
    <()>::get_translate::<{ Ptr(f as *const ()) }>();
}

/// 