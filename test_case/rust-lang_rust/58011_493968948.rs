rs
#![feature(existential_type)]

pub trait A {}
impl A for u8 {}

existential type T: A;

fn f() -> T {
    0u8
}
