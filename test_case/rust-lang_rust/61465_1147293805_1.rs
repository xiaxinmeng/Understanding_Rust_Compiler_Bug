rs
#![allow(incomplete_features, dead_code)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Num {
    One = 1,
    Two = 2,
}

fn foo<const NUM: Num>() {
    const NUM: usize = NUM as usize;
}

fn bar<const NUM: Num>() {
    foo::<{ NUM }>();
}

fn main() {
    bar::<{ Num::One }>();
}
