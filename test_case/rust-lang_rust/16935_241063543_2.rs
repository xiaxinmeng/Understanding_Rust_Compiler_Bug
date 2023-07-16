 rust
#![feature(prelude_import)]
#![no_std]
#![feature(plugin)]
#![plugin(synext)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

fn main() { std::io::stdout().write("one") }
