rust
#![feature(proc_macro)]

extern crate codegen;
use codegen::demo;

#[demo] struct X;

fn main() { }
