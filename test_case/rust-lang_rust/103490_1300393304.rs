rust
#![feature(unboxed_closures)]

#[rustc_paren_sugar]
pub trait Tr<'a, T> { type Output; }

pub fn f(_: impl Tr(i32)) {} //~ ERROR this trait takes 1 lifetime argument but
                             //        0 lifetime arguments were supplied
