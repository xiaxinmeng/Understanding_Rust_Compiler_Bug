 rust
#![warn(variant_size_differences)]

#![feature(phase)]

#[phase(plugin)]
extern crate peg_syntax_ext;

peg! parser("")

fn main() {}
