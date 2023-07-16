rust
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
const fn foo() -> u32 { 1 }
fn main() { loop  { } }
