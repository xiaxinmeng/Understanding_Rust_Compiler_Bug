
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
fn main() {
    ::io::_print(::std::fmt::Arguments::new_v1(&["rust\n"],
                                               &match () { () => [], }));
}
